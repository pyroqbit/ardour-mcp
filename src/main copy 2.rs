#![feature(more_qualified_paths)]

use anyhow::{Context, Result};
use async_trait::async_trait;
use schemars::JsonSchema;
use nannou_osc as osc;
use osc::Type as OscType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info, warn, instrument};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::writer::Tee;
use tracing_appender;

// Attempting direct paths, assuming modules are public via features
use rmcp::ServiceError;
use rmcp::error::ErrorCodeTrait; // Direct path
use rmcp::error::McpError as RmcpErrorPayload; // Direct path, aliased for clarity
use rmcp::model::{
    CallToolRequest,
    CallToolResult,
    ListResourcesRequest,
    ListResourcesResult,
    ReadResourceRequest,
    ReadResourceResult,
    Resource,
    ServerInfo,
    ErrorData,
    CallMethodRequest,
    CallMethodResult,
    WriteResourceParams,
    WriteResourceResult,
    Annotated,
    RawResource,
    InitializeResult,
    InitializeParams,
    ServerCapabilities,
};
use rmcp::service::{RequestContext, Service as McpService, ServiceRole, RoleServer, Peer as ServicePeer};
use rmcp::transport::stdio_transport; // Direct path if transport module is pub
use rmcp::server::Server;             // Direct path if server module is pub

const OSC_RECEIVE_PORT: u16 = 3819; // Port Ardour sends OSC feedback to
const ARDOUR_OSC_HOST_PORT: &str = "127.0.0.1:3819"; // Ardour's default OSC port (actually Ardour listens on 3819 by default for commands too)
                                                   // We send to Ardour on 3819, Ardour sends feedback to this app on OSC_RECEIVE_PORT (e.g. 9001)
                                                   // For now, let's assume Ardour listens on 3819 for commands.
                                                   // And this server listens on, say, 9001 for feedback.
const THIS_SERVER_LISTEN_PORT: u16 = 9001; // Port this server listens on for feedback from Ardour
const ARDOUR_COMMAND_TARGET_ADDR: &str = "127.0.0.1:3819";

// Definition of ArdourErrorCode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, JsonSchema)]
pub enum ArdourErrorCode {
    NotFound,
    InvalidParameters,
    OscSendError,
    InternalError,
    UnsupportedMethod,
    ResourceNotReadable,
    OperationFailed,
    ArdourError,
}

impl std::fmt::Display for ArdourErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Define the TrackInfo struct (already present in user's code)
#[derive(Serialize, Deserialize, Debug, Clone, schemars::JsonSchema)]
pub struct TrackInfo {
    id: i32,
    name: String,
    track_type: String, // e.g., "audio", "midi"
    soloed: bool,
    muted: bool,
    rec_enabled: bool,
    // Add other relevant fields
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocateToolArgs {
    spos: i64, // Sample position
    roll: Option<i32>, // Unknown, defaults to 0 based on docs
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SetTrackSoloArgs {
    rid: i32,    // Route ID
    solo_st: i32, // Solo state (0 or 1)
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SetTrackRecEnableArgs {
    rid: i32,    // Route ID
    rec_st: i32, // Record arm state (0 or 1)
}

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct SetTrackGainAbsArgs {
    rid: i32,     // Route ID
    gain_abs: f32, // Absolute gain (0.0 to 2.0 typically)
}

#[derive(Clone)]
struct ArdourState {
    osc_sender: mpsc::Sender<(String, Vec<OscType>)>,
    osc_feedback_tx: mpsc::Sender<osc::Packet>, // To send feedback from OSC receiver to service
    tracks: Arc<Mutex<HashMap<i32, TrackInfo>>>,
}

struct ArdourService {
    state: ArdourState,
    // OSC sender for commands to Ardour
    osc_cmd_sender: osc::Sender<osc::Connected>,
}

// Helper to send OSC commands (modified to use the stored sender)
impl ArdourService {
    async fn send_osc_command(&self, address: String, args: Vec<OscType>) -> Result<(), ServiceError> {
        debug!(?address, ?args, "Attempting to send OSC command");
        if let Err(e) = self.osc_cmd_sender.send((address.as_str(), args.clone())) {
            error!("Failed to send OSC command: {}", e);
            return Err(ServiceError::McpError(RmcpErrorPayload::new(
                ArdourErrorCode::OscSendError,
                Some(ErrorData::new(ArdourErrorCode::OscSendError.to_string(), format!("Failed to send OSC command: {}", e), None)),
            )));
        }
        info!("Successfully sent OSC command: {} with args {:?}", address, args);
        Ok(())
    }
    
    async fn send_osc_setup_to_ardour(&self) -> Result<(), ServiceError> {
        info!("Configuring Ardour OSC feedback to 127.0.0.1:{}", THIS_SERVER_LISTEN_PORT);
        self.send_osc_command(
            "/set_surface/feedback_host_port".to_string(), 
            vec![
                OscType::Int(0),
                OscType::String("127.0.0.1".to_string()), 
                OscType::Int(THIS_SERVER_LISTEN_PORT as i32)
            ]
        ).await.map_err(|e_service|{
            error!("Failed to set feedback host/port during setup: {:?}", e_service);
            e_service
        })?;
        
        self.send_osc_command("/strip/list".to_string(), vec![]).await.map_err(|e_service|{
            error!("Failed to send /strip/list during setup: {:?}", e_service);
            e_service
        })?;
        Ok(())
    }

    fn list_tools_as_resources(&self) -> Vec<Annotated<RawResource>> {
        let locate_schema = schemars::schema_for!(LocateToolArgs);
        let set_track_solo_schema = schemars::schema_for!(SetTrackSoloArgs);
        let set_track_rec_enable_schema = schemars::schema_for!(SetTrackRecEnableArgs);
        let set_track_gain_abs_schema = schemars::schema_for!(SetTrackGainAbsArgs);

        vec![
            Annotated {
                raw: RawResource {
                    id: "tool_locate".to_string(),
                    name: Some("Locate".to_string()),
                    description: Some("Set transport position in Ardour.".to_string()),
                    mime_type: Some("application/json".to_string()),
                    methods: Some(vec!["call".to_string()]),
                    properties: Some(json!({ "parameters_schema": locate_schema })),
                    ..Default::default()
                },
                annotations: None,
            },
            Annotated {
                raw: RawResource {
                    id: "tool_set_track_solo".to_string(),
                    name: Some("Set Track Solo".to_string()),
                    description: Some("Set solo state for a track/bus in Ardour.".to_string()),
                    mime_type: Some("application/json".to_string()),
                    methods: Some(vec!["call".to_string()]),
                    properties: Some(json!({ "parameters_schema": set_track_solo_schema })),
                    ..Default::default()
                },
                annotations: None,
            },
            Annotated {
                raw: RawResource {
                    id: "tool_set_track_rec_enable".to_string(),
                    name: Some("Set Track Record Arm".to_string()),
                    description: Some("Set record arm state for a track/bus in Ardour.".to_string()),
                    mime_type: Some("application/json".to_string()),
                    methods: Some(vec!["call".to_string()]),
                    properties: Some(json!({ "parameters_schema": set_track_rec_enable_schema })),
                    ..Default::default()
                },
                annotations: None,
            },
            Annotated {
                raw: RawResource {
                    id: "tool_set_track_gain_abs".to_string(),
                    name: Some("Set Track Gain (Absolute)".to_string()),
                    description: Some("Set absolute gain for a track/bus in Ardour (0.0 to 2.0).".to_string()),
                    mime_type: Some("application/json".to_string()),
                    methods: Some(vec!["call".to_string()]),
                    properties: Some(json!({ "parameters_schema": set_track_gain_abs_schema })),
                    ..Default::default()
                },
                annotations: None,
            },
        ]
    }
    
    #[instrument(skip(self))]
    async fn locate_tool(&self, args: LocateToolArgs) -> Result<serde_json::Value, ServiceError> {
        info!("Calling locate_tool with args: {:?}", args);
        let roll = args.roll.unwrap_or(0);
        self.send_osc_command(
            "/ardour/locate".to_string(),
            vec![OscType::Long(args.spos), OscType::Int(roll)],
        )
        .await?;
        Ok(json!({"status": "ok", "message": "Locate command sent."}))
    }

    #[instrument(skip(self))]
    async fn set_track_solo_tool(&self, args: SetTrackSoloArgs) -> Result<serde_json::Value, ServiceError> {
        info!("Calling set_track_solo_tool with args: {:?}", args);
        if !(args.solo_st == 0 || args.solo_st == 1) {
            return Err(ServiceError::McpError(RmcpErrorPayload::new(
                ArdourErrorCode::InvalidParameters,
                Some(ErrorData::new("solo_st must be 0 or 1".to_string(), None)),
            )));
        }
        self.send_osc_command(
            "/ardour/routes/solo".to_string(),
            vec![OscType::Int(args.rid), OscType::Int(args.solo_st)],
        ).await?;
        Ok(json!({"status": "ok", "message": "Set track solo command sent."}))
    }
    
    #[instrument(skip(self))]
    async fn set_track_rec_enable_tool(&self, args: SetTrackRecEnableArgs) -> Result<serde_json::Value, ServiceError> {
        info!("Calling set_track_rec_enable_tool with args: {:?}", args);
        if !(args.rec_st == 0 || args.rec_st == 1) {
            return Err(ServiceError::McpError(RmcpErrorPayload::new(
                ArdourErrorCode::InvalidParameters,
                Some(ErrorData::new("rec_st must be 0 or 1".to_string(), None)),
            )));
        }
        self.send_osc_command(
            "/ardour/routes/recenable".to_string(),
            vec![OscType::Int(args.rid), OscType::Int(args.rec_st)],
        ).await?;
        Ok(json!({"status": "ok", "message": "Set track rec_enable command sent."}))
    }

    #[instrument(skip(self))]
    async fn set_track_gain_abs_tool(&self, args: SetTrackGainAbsArgs) -> Result<serde_json::Value, ServiceError> {
        info!("Calling set_track_gain_abs_tool with args: {:?}", args);
        if !(args.gain_abs >= 0.0 && args.gain_abs <= 2.0) {
            return Err(ServiceError::McpError(RmcpErrorPayload::new(
                ArdourErrorCode::InvalidParameters,
                Some(ErrorData::new("gain_abs must be between 0.0 and 2.0".to_string(), None)),
            )));
        }
        self.send_osc_command(
            "/ardour/routes/gainabs".to_string(),
            vec![OscType::Int(args.rid), OscType::Float(args.gain_abs)],
        ).await?;
        Ok(json!({"status": "ok", "message": "Set track gain_abs command sent."}))
    }

    pub fn new(osc_cmd_sender: osc::Sender<osc::Connected>, osc_feedback_tx: mpsc::Sender<osc::Packet>) -> Self {
        let (_dummy_tx, _dummy_rx) = mpsc::channel(1);
        let state = ArdourState {
            osc_sender: _dummy_tx, 
            osc_feedback_tx,
            tracks: Arc::new(Mutex::new(HashMap::new())),
        };
        ArdourService { state, osc_cmd_sender }
    }

    // ---- Internal helper methods for rmcp::Service ----
    async fn internal_list_resources(
        &self,
        _ctx: RequestContext<RoleServer>,
        _params: ListResourcesRequest,
    ) -> Result<ListResourcesResult, ServiceError> {
        info!("internal_list_resources called with ctx");
        let mut resources = self.list_tools_as_resources();
        let tracks_guard = self.state.tracks.lock().await;
        for (id, track_info) in tracks_guard.iter() {
            let track_schema = schemars::schema_for!(crate::TrackInfo);
            resources.push(Resource {
                id: format!("track_{}", id),
                name: Some(track_info.name.clone()),
                description: Some(format!("Ardour track: {}", track_info.name)),
                mime_type: Some("application/json".to_string()),
                methods: Some(vec!["read".to_string()]),
                properties: Some(json!({ "schema": track_schema })),
                ..Default::default()
            });
        }
        Ok(ListResourcesResult { resources })
    }

    async fn internal_read_resource(
        &self,
        _ctx: RequestContext<RoleServer>,
        params: ReadResourceRequest,
    ) -> Result<ReadResourceResult, ServiceError> {
        info!("internal_read_resource called for ID: {} with ctx", params.resource_id);
        if params.resource_id.starts_with("track_") {
            let track_id_str = params.resource_id.trim_start_matches("track_");
            if let Ok(track_id) = track_id_str.parse::<i32>() {
                let tracks_guard = self.state.tracks.lock().await;
                if let Some(track_info) = tracks_guard.get(&track_id) {
                    let data_vec = serde_json::to_vec(track_info).map_err(|e| {
                        error!("Serialization error for track {}: {}", track_id, e);
                        ServiceError::McpError(RmcpErrorPayload::new(
                            ArdourErrorCode::InternalError,
                            Some(ErrorData::new(format!("Serialization error: {}", e), None)),
                        ))
                    })?;
                    return Ok(ReadResourceResult {
                        mime_type: "application/json".to_string(),
                        data: data_vec,
                    });
                }
            }
        }
        Err(ServiceError::McpError(RmcpErrorPayload::new(
            ArdourErrorCode::NotFound,
            Some(ErrorData::new(format!("Resource ID {} not found or not readable", params.resource_id), None)),
        )))
    }
    
    async fn internal_call_method(
        &self,
        _ctx: RequestContext<RoleServer>,
        method_id: String,
        params_value: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, ServiceError> {
        info!("internal_call_method called for method_id: {}, params: {:?} with ctx", method_id, params_value);
        
        let deserialize_params = |p_val, tool_name| {
            p_val.ok_or_else(|| ServiceError::McpError(RmcpErrorPayload::new(ArdourErrorCode::InvalidParameters, Some(ErrorData::new(format!("Missing parameters for {}", tool_name), None)))))
                 .and_then(|v| serde_json::from_value(v).map_err(|e| ServiceError::McpError(RmcpErrorPayload::new(ArdourErrorCode::InvalidParameters, Some(ErrorData::new(format!("Invalid parameters for {}: {}", tool_name, e), None))))))
        };

        match method_id.as_str() {
            "tool_locate" => {
                let args: LocateToolArgs = deserialize_params(params_value, "locate tool")?;
                self.locate_tool(args).await
            }
            "tool_set_track_solo" => {
                let args: SetTrackSoloArgs = deserialize_params(params_value, "set_track_solo tool")?;
                self.set_track_solo_tool(args).await
            }
            "tool_set_track_rec_enable" => {
                let args: SetTrackRecEnableArgs = deserialize_params(params_value, "set_track_rec_enable tool")?;
                self.set_track_rec_enable_tool(args).await
            }
            "tool_set_track_gain_abs" => {
                let args: SetTrackGainAbsArgs = deserialize_params(params_value, "set_track_gain_abs tool")?;
                self.set_track_gain_abs_tool(args).await
            }
            _ => Err(ServiceError::McpError(RmcpErrorPayload::new(
                ArdourErrorCode::UnsupportedMethod,
                Some(ErrorData::new(format!("Unsupported method_id: {}", method_id), None)),
            ))),
        }
    }
}

#[async_trait]
impl McpService<RoleServer> for ArdourService {
    async fn get_info(&self) -> InitializeResult {
        info!("get_info called");
        let server_info_val = ServerInfo {
            name: "ardour-mcp-server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: Some("MCP server for Ardour DAW control via OSC".to_string()),
            authors: Some(env!("CARGO_PKG_AUTHORS").to_string()),
            capabilities: Some(ServerCapabilities { /* .. fill this .. */ ..Default::default() }),
            host_id: None,
            host_name: None,
            tags: None,
            url: None,
            schema_url: None,
            resources: self.list_tools_as_resources().into_iter().map(|ar| ar.raw).collect(),
        };
        InitializeResult {
            protocol_version: "0.1.0".to_string(),
            server_info: Some(server_info_val),
            instructions: None,
        }
    }

    async fn handle_request(&self,
        request: <RoleServer as ServiceRole>::PeerReq,
        ctx: RequestContext<RoleServer>,
    ) -> Result<<RoleServer as ServiceRole>::PeerResp, ErrorData> {
        info!(?request, ?ctx, "handle_request called - LIKELY OBSOLETE FOR RMCP 0.1.5");
        
        let map_service_err_to_model_error_data = |service_err: ServiceError| -> ErrorData {
            match service_err {
                ServiceError::McpError(mcp_payload) => {
                    match mcp_payload.data {
                        Some(existing_error_data) => ErrorData::new(existing_error_data.code, existing_error_data.message, existing_error_data.data),
                        None => ErrorData::new(mcp_payload.code.to_string(), format!("Error: {}", mcp_payload.code), None)
                    }
                }
                ServiceError::Internal(s) => ErrorData::new(s, None),
                ServiceError::CallNotSupported => ErrorData::new("Call not supported".to_string(), None),
                ServiceError::ResourceNotReadable => ErrorData::new("Resource not readable".to_string(), None),
                ServiceError::ResourceNotWritable => ErrorData::new("Resource not writable".to_string(), None),
                _ => ErrorData::new("UNKNOWN_ERROR_CODE".to_string(), "Unknown service error".to_string(), None),
            }
        };

        match request {
            <RoleServer as ServiceRole>::PeerReq::GetInfo(_params) => { 
                Err(ErrorData::new("TEMP_ERR".to_string(),"GetInfo via handle_request is obsolete".to_string(),None))
            }
            <RoleServer as ServiceRole>::PeerReq::ListResources(params) => {
                self.internal_list_resources(ctx, params).await
                    .map(<RoleServer as ServiceRole>::PeerResp::ListResources)
                    .map_err(map_service_err_to_model_error_data)
            }
            <RoleServer as ServiceRole>::PeerReq::ReadResource(params) => {
                self.internal_read_resource(ctx, params).await
                    .map(<RoleServer as ServiceRole>::PeerResp::ReadResource)
                    .map_err(map_service_err_to_model_error_data)
            }
            <RoleServer as ServiceRole>::PeerReq::CallMethod(call_params @ CallToolRequest { .. }) => {
                self.internal_call_method(ctx, call_params.resource_id.clone(), call_params.params.clone())
                    .await
                    .map(|result| <RoleServer as ServiceRole>::PeerResp::CallMethod(CallToolResult { result: Some(result) }))
                    .map_err(map_service_err_to_model_error_data)
            }
            <RoleServer as ServiceRole>::PeerReq::WriteResource(params) => { 
                 warn!(?params, "WriteResource requested but not implemented");
                 Err(ErrorData::new("WriteResource not implemented".to_string(), None))
            }
            <RoleServer as ServiceRole>::PeerReq::Subscribe(params) => {
                 warn!(?params, "Subscribe requested but not implemented");
                 Err(ErrorData::new("Subscribe not implemented".to_string(), None))
            }
            <RoleServer as ServiceRole>::PeerReq::Unsubscribe(params) => {
                 warn!(?params, "Unsubscribe requested but not implemented");
                 Err(ErrorData::new("Unsubscribe not implemented".to_string(), None))
            }
             _ => {
                warn!("Unhandled PeerRequest type");
                Err(ErrorData::new("TEMP_ERR".to_string(), "handle_request is largely obsolete".to_string(), None))
            }
        }
    }
    
    async fn handle_notification(&self,
        notification: <RoleServer as ServiceRole>::PeerNot,
    ) -> Result<(), ErrorData> {
        info!(?notification, "handle_notification called");
        Ok(())
    }

    async fn get_peer(&self) -> Option<ServicePeer<RoleServer>> {
        warn!("get_peer (todo) called, but not implemented");
        todo!()
    }

    async fn set_peer(&mut self, peer: ServicePeer<RoleServer>) -> Result<(), ErrorData> {
        warn!(?peer, "set_peer (todo) called, but not implemented");
        todo!()
    }
}

// OSC receiver task
async fn run_osc_receiver(
    bind_addr: String,
    feedback_tx: mpsc::Sender<osc::Packet>, 
) {
    info!("Starting OSC receiver on {}", bind_addr);
    let mut receiver_opt: Option<osc::Receiver> = None;

    // Initial bind attempt
    match osc::Receiver::bind(&bind_addr).await {
        Ok(r) => {
            info!("OSC receiver bound to {}", bind_addr);
            receiver_opt = Some(r);
        }
        Err(e) => {
            error!("Failed to bind OSC receiver initially to {}: {}. Will retry in loop.", bind_addr, e);
        }
    };

    loop {
        if receiver_opt.is_none() {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // Wait before retrying to bind
            match osc::Receiver::bind(&bind_addr).await {
                Ok(r) => {
                    info!("OSC receiver successfully re-bound to {}", bind_addr);
                    receiver_opt = Some(r);
                }
                Err(e) => {
                    error!("Failed to re-bind OSC receiver to {}: {}. Will retry.", bind_addr, e);
                    continue; // Try again in the next loop iteration
                }
            };
        }

        if let Some(ref mut receiver) = receiver_opt {
            match receiver.try_recv().await { // Assuming try_recv is available and async
                Ok(Some((packet, source_addr))) => {
                    debug!("OSC packet received from {:?}: {:?}", source_addr, packet);
                    if let Err(e) = feedback_tx.send(packet).await {
                         error!("Failed to send OSC packet to feedback channel: {}", e);
                    }
                }
                Ok(None) => {
                    // No packet available yet, yield or sleep briefly
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
                Err(e) => {
                    warn!("OSC receiver error: {}. Resetting receiver.", e);
                    receiver_opt = None; // Trigger re-bind in the next iteration
                }
            }
        } else {
            // Should not happen if logic is correct, but as a fallback, ensure we try to rebind
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

// Handler for received OSC packets (called from a task that processes `osc_feedback_rx`)
async fn handle_osc_packet(packet: osc::Packet, tracks: Arc<Mutex<HashMap<i32, crate::TrackInfo>>>) {
    match packet {
        osc::Packet::Message(msg) => {
            info!(addr = %msg.addr, args = ?msg.args, "Received OSC Message");
            
            // Example: /ardour/strip/name <route_id:int> <name:string>
            if msg.addr == "/ardour/strip/name" {
                if msg.args.len() == 2 {
                    if let (Some(OscType::Int(id)), Some(OscType::String(name))) = (msg.args.get(0), msg.args.get(1)) {
                        let mut tracks_guard = tracks.lock().await;
                        let entry = tracks_guard.entry(*id).or_insert_with(|| crate::TrackInfo {
                            id: *id,
                            name: name.clone(),
                            track_type: "unknown".to_string(),
                            soloed: false,
                            muted: false,
                            rec_enabled: false,
                        });
                        entry.name = name.clone();
                        info!(track_id = id, track_name = name, "Updated track name");
                    }
                } else {
                    warn!(addr=%msg.addr, "Received /ardour/strip/name with unexpected args length: {}", msg.args.len());
                }
            }
            // Add more handlers here, e.g., for solo, mute, rec_arm status, meter levels, etc.
            // /ardour/routes/solo <route_id:int> <solo_state:int (0 or 1)>
            else if msg.addr == "/ardour/routes/solo" && msg.args.len() == 2 {
                 if let (Some(OscType::Int(id)), Some(OscType::Int(solo_state))) = (msg.args.get(0), msg.args.get(1)) {
                    let mut tracks_guard = tracks.lock().await;
                    if let Some(track) = tracks_guard.get_mut(id) {
                        track.soloed = *solo_state == 1;
                        info!(track_id = id, solo = track.soloed, "Updated track solo state");
                    } else {
                        warn!(track_id = id, "Received solo state for unknown track");
                    }
                }
            }
            // Add handlers for /ardour/routes/mute, /ardour/routes/recenable

        }
        osc::Packet::Bundle(bundle) => {
            info!(num_packets = bundle.content.len(), "Received OSC Bundle");
            for packet_in_bundle in bundle.content {
                // Avoid cloning Arc for every packet in a potentially large bundle if possible,
                // but for simplicity and correctness, cloning Arc is safe.
                handle_osc_packet(packet_in_bundle, Arc::clone(&tracks)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Tracing setup (should be already corrected by previous edits)
    let log_file_prefix = format!("ardour_mcp_server_{}", chrono::Local::now().format("%Y-%m-%d"));
    let file_appender = tracing_appender::rolling::daily("logs", log_file_prefix);
    let (non_blocking_file, _guard_file) = tracing_appender::non_blocking(file_appender);
    let stderr_writer = std::io::stderr;
    let combined_writer = Tee::new(stderr_writer, non_blocking_file);
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("ardour_mcp_server=trace".parse().unwrap())
            .add_directive("rmcp=info".parse().unwrap())
        )
        .with_writer(combined_writer)
        .with_ansi(false)
        .init();

    info!("Starting Ardour MCP Server...");

    let ardour_command_target_socket_addr: SocketAddr = ARDOUR_COMMAND_TARGET_ADDR.parse()
        .context("Failed to parse Ardour command target address")?;
    let osc_cmd_sender = osc::Sender::connect(ardour_command_target_socket_addr).await
        .context("Failed to connect OSC command sender to Ardour")?;
    info!("OSC command sender connected to Ardour at {}", ARDOUR_COMMAND_TARGET_ADDR);

    let (osc_feedback_tx, mut osc_feedback_rx) = mpsc::channel::<osc::Packet>(100);

    let service = Arc::new(ArdourService::new(osc_cmd_sender, osc_feedback_tx.clone()));

    if let Err(e) = service.send_osc_setup_to_ardour().await {
         error!("Failed to send initial OSC setup to Ardour: {:?}. This might affect feedback.", e);
    }

    let listen_addr = format!("0.0.0.0:{}", THIS_SERVER_LISTEN_PORT);
    tokio::spawn(run_osc_receiver(listen_addr, osc_feedback_tx));

    let tracks_for_handler = Arc::clone(&service.state.tracks);
    tokio::spawn(async move {
        info!("OSC feedback processing task started.");
        while let Some(packet) = osc_feedback_rx.recv().await {
            handle_osc_packet(packet, Arc::clone(&tracks_for_handler)).await;
        }
        info!("OSC feedback processing task finished.");
    });

    let (transport, client_task) = stdio_transport();
    info!("stdio_transport initialized.");

    tokio::spawn(async move {
        info!("rmcp client_task (stdio handler) starting.");
        if let Err(e) = client_task.await {
            error!("rmcp client_task (stdio handler) exited with error: {:?}", e);
        } else {
            info!("rmcp client_task (stdio handler) finished.");
        }
    });
    
    info!("Serving ArdourService over stdio transport...");
    
    let ardour_service_arc = service as Arc<dyn McpService<RoleServer> + Send + Sync>;
    let server = Server::new(ardour_service_arc);
    server.serve(transport).await.context("MCP Server failed")?;

    info!("Ardour MCP Server shut down.");
    Ok(())
} 