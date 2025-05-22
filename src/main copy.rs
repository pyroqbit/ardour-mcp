use anyhow::Result;
use rmcp::model::{
    AnnotateAble,
    CallToolResult,
    Content,
    GetPromptRequestParam,
    GetPromptResult,
    Implementation,
    ListPromptsResult,
    ListResourcesResult,
    PaginatedRequestParam,
    ProtocolVersion,
    RawResource,
    ReadResourceRequestParam,
    ReadResourceResult,
    Resource,
    ServerCapabilities,
    ServerInfo,
}; 
use rmcp::{
    Error as McpError, 
    RoleServer,
    ServerHandler,
    ServiceExt,
    service::RequestContext,
    tool,
    transport::stdio,
};

use nannou_osc as osc;
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json; 
use std::sync::Arc;
use tokio::sync::Mutex;

// Add imports for file logging
use std::fs::OpenOptions;
use std::path::Path;
// Import for combining writers
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::fmt::writer::Tee;
use tracing_subscriber::EnvFilter;

use ardour_mcp_server::TrackInfo;

const ARDOUR_OSC_TARGET_ADDR: &str = "127.0.0.1:3819";
#[allow(dead_code)]
const MCP_SERVER_OSC_LISTEN_ADDR: &str = "127.0.0.1:9099";

const ARDOUR_OSC_TCP_TIMEOUT_MS: u64 = 2000;

#[derive(Debug, Clone, PartialEq)]
enum PlaybackStatus {
    Playing,
    Stopped,
    Unknown,
}

#[derive(Debug, Clone)]
struct ArdourState {
    playback_status: PlaybackStatus,
    strip_list: Vec<TrackInfo>,
}

impl ArdourState {
    fn new() -> Self {
        Self {
            playback_status: PlaybackStatus::Unknown,
            strip_list: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct ArdourService {
    osc_sender: Arc<Mutex<Option<osc::Sender<osc::Connected>>>>,
    ardour_state: Arc<Mutex<ArdourState>>,
    settings: Settings,
}

impl ArdourService {
    async fn new(settings: Settings) -> Self {
        let sender = Self::connect_osc(&settings).await;
        ArdourService {
            osc_sender: Arc::new(Mutex::new(sender)),
            ardour_state: Arc::new(Mutex::new(ArdourState::new())),
            settings,
        }
    }

    async fn connect_osc(settings: &Settings) -> Option<osc::Sender<osc::Connected>> {
        tracing::info!("Attempting to create OSC sender for Ardour at {}", ARDOUR_OSC_TARGET_ADDR);
        let sender = osc::sender()
            .map_err(|e| anyhow::anyhow!("Failed to create OSC sender builder: {}", e))?
            .connect(ARDOUR_OSC_TARGET_ADDR)
            .map_err(|e| anyhow::anyhow!("Failed to prepare OSC sender for {}: {}", ARDOUR_OSC_TARGET_ADDR, e))?;
        tracing::info!("OSC sender created and connected to Ardour at {}", ARDOUR_OSC_TARGET_ADDR);
        Some(sender)
    }

    async fn ensure_connected(&self) -> Option<osc::Sender<osc::Connected>> {
        let sender = self.osc_sender.lock().await;
        sender.clone()
    }

    async fn send_osc_message(&self, address: &str, args: Option<Vec<osc::Type>>) -> Result<()> {
        let osc_sender_clone = Arc::clone(&self.osc_sender);
        let owned_address = address.to_string();
        tokio::task::spawn_blocking(move || {
            let sender_guard = osc_sender_clone.blocking_lock();
            let msg_args = args.unwrap_or_default();
            let msg = osc::Message { addr: owned_address, args: msg_args };
            sender_guard.send(msg).map_err(|e| anyhow::anyhow!("Failed to send OSC message: {}", e))
        }).await??; 
        Ok(())
    }

    async fn send_osc_setup_to_ardour(&self) -> Result<()> {
        tracing::info!("Sending /set_surface to Ardour to enable OSC feedback.");

        // Extract port from MCP_SERVER_OSC_LISTEN_ADDR ("127.0.0.1:9099")
        let parts: Vec<&str> = MCP_SERVER_OSC_LISTEN_ADDR.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid MCP_SERVER_OSC_LISTEN_ADDR format: {}", MCP_SERVER_OSC_LISTEN_ADDR));
        }
        let feedback_port_num: i32 = parts[1].parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse port from MCP_SERVER_OSC_LISTEN_ADDR: {}", e))?;

        tracing::info!("Targeting feedback port: {}", feedback_port_num);

        // Construct the OSC message for /set_surface with all parameters
        // /set_surface i:0 i:159 i:1 i:0 i:0 i:0 i:<feedback_port_num> i:0 i:0
        // According to Ardour manual: /set_surface bank_size strip_types feedback fadermode ...
        // Our mapping: bank_size=0 (1st arg), strip_types=159 (2nd arg), feedback=7 (3rd arg)
        let osc_args = vec![
            osc::Type::Int(0),    // surface_id (bank_size = 0 for no banking / infinite)
            osc::Type::Int(159),  // strip_types (Audio, MIDI, Busses, VCAs, Master, Monitor etc.)
            osc::Type::Int(7),    // feedback (1=button_status | 2=variable_values | 4=ssid_in_path)
            osc::Type::Int(0),    // fadermode (0 = dB)
            osc::Type::Int(0),    // send_page_size
            osc::Type::Int(0),    // plugin_page_size
            osc::Type::Int(feedback_port_num), // feedback_port
            osc::Type::Int(0),    // linkset (n_strips in old comment)
            osc::Type::Int(0)     // linkid (n_sends_per_strip in old comment)
        ];

        match self.send_osc_message("/set_surface", Some(osc_args)).await {
            Ok(_) => {
                tracing::info!("/set_surface command sent successfully to Ardour.");
                Ok(())
            }
            Err(e) => {
                let err_msg = format!("Failed to send /set_surface OSC message to Ardour: {}", e);
                tracing::error!("{}", err_msg);
                Err(anyhow::anyhow!(err_msg))
            }
        }
    }

    #[tool(tool_box)] 
    async fn transport_play_tool(&self) -> Result<CallToolResult, McpError> { 
        tracing::info!("Executing transport_play_tool");
        match self.send_osc_message("/transport_play", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Playback started")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "transport_stop", description = "Stops Ardour playback.")]
    async fn transport_stop_tool(&self) -> Result<CallToolResult, McpError> { 
        tracing::info!("Executing transport_stop_tool");
        match self.send_osc_message("/transport_stop", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Playback stopped")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "goto_start", description = "Moves the playhead to the session start.")]
    async fn goto_start_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing goto_start_tool");
        match self.send_osc_message("/goto_start", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Playhead moved to start")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "goto_end", description = "Moves the playhead to the session end.")]
    async fn goto_end_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing goto_end_tool");
        match self.send_osc_message("/goto_end", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Playhead moved to end")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "loop_toggle", description = "Toggles loop playback mode.")]
    async fn loop_toggle_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing loop_toggle_tool");
        match self.send_osc_message("/loop_toggle", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Loop mode toggled")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "undo", description = "Undoes the last action.")]
    async fn undo_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing undo_tool");
        match self.send_osc_message("/undo", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Undo action performed")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "redo", description = "Redoes the last undone action.")]
    async fn redo_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing redo_tool");
        match self.send_osc_message("/redo", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Redo action performed")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "toggle_punch_in", description = "Toggles the Punch In state.")]
    async fn toggle_punch_in_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing toggle_punch_in_tool");
        match self.send_osc_message("/toggle_punch_in", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Punch In toggled")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "toggle_punch_out", description = "Toggles the Punch Out state.")]
    async fn toggle_punch_out_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing toggle_punch_out_tool");
        match self.send_osc_message("/toggle_punch_out", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Punch Out toggled")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "rec_enable_toggle", description = "Toggles the master record enable or selected track record enable.")]
    async fn rec_enable_toggle_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing rec_enable_toggle_tool");
        match self.send_osc_message("/rec_enable_toggle", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Record Enable toggled")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "toggle_all_rec_enables", description = "Toggles the record enable state for ALL tracks.")]
    async fn toggle_all_rec_enables_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing toggle_all_rec_enables_tool");
        match self.send_osc_message("/toggle_all_rec_enables", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("All Record Enables toggled")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "ffwd", description = "Fast forwards the transport.")]
    async fn ffwd_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing ffwd_tool");
        match self.send_osc_message("/ffwd", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Fast Forward activated")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "rewind", description = "Rewinds the transport.")]
    async fn rewind_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing rewind_tool");
        match self.send_osc_message("/rewind", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Rewind activated")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "add_marker", description = "Adds a location marker at the current playhead position.")]
    async fn add_marker_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing add_marker_tool");
        match self.send_osc_message("/add_marker", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Marker added")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "next_marker", description = "Moves the playhead to the next location marker.")]
    async fn next_marker_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing next_marker_tool");
        match self.send_osc_message("/next_marker", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Moved to next marker")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "prev_marker", description = "Moves the playhead to the previous location marker.")]
    async fn prev_marker_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing prev_marker_tool");
        match self.send_osc_message("/prev_marker", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Moved to previous marker")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "save_state", description = "Saves the current session state.")]
    async fn save_state_tool(&self) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing save_state_tool");
        match self.send_osc_message("/save_state", None).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text("Session state saved")])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!("OSC send error: {}",e))])),
        }
    }

    #[tool(name = "set_track_mute", description = "Sets the mute state of a specific track.")]
    async fn set_track_mute_tool(
        &self,
        #[schemars(description = "Arguments for setting track mute state. Requires 'rid' (integer) and 'mute_state' (boolean).")]
        #[tool(aggr)] args: SetTrackMuteArgs 
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_mute_tool with args: {:?}", args);

        let osc_mute_state = if args.mute_state { 1i32 } else { 0i32 };
        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Int(osc_mute_state)];
        
        match self.send_osc_message("/ardour/routes/mute", Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} mute state set to {}",
                args.rid, args.mute_state
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for set_track_mute: {}",
                e
            ))])),
        }
    }

    #[tool(name = "set_transport_speed", description = "Sets Ardour's transport speed. Valid range: -8.0 to 8.0.")]
    async fn set_transport_speed_tool(
        &self,
        #[schemars(description = "Argument for setting transport speed. Requires 'speed' (float).")]
        #[tool(aggr)] args: SetTransportSpeedArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Executing set_transport_speed_tool with speed: {}",
            args.speed
        );

        if args.speed < -8.0 || args.speed > 8.0 {
            tracing::warn!("Invalid transport speed: {}. Must be between -8.0 and 8.0.", args.speed);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid transport speed: {}. Must be between -8.0 and 8.0.",
                args.speed
            ))]));
        }

        let osc_args = vec![osc::Type::Float(args.speed)];
        match self.send_osc_message("/set_transport_speed", Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Transport speed set to {}",
                args.speed
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for /set_transport_speed: {}",
                e
            ))])),
        }
    }

    #[tool(name = "locate", description = "Locates the playhead to a specific sample position and optionally starts playback.")]
    async fn locate_tool(
        &self,
        #[schemars(description = "Arguments for locating the playhead. Requires 'spos' (integer samples) and 'roll' (integer 0 or 1).")]
        #[tool(aggr)] args: LocateToolArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing locate_tool with spos: {}, roll: {}", args.spos, args.roll);
        let osc_args = vec![osc::Type::Long(args.spos), osc::Type::Int(args.roll)];
        match self.send_osc_message("/ardour/locate", Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Located to sample {} with roll state {}",
                args.spos, args.roll
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for /ardour/locate: {}",
                e
            ))])),
        }
    }

    #[tool(name = "set_track_solo", description = "Sets the solo state of a specific track.")]
    async fn set_track_solo_tool(
        &self,
        #[schemars(description = "Arguments for setting track solo state. Requires 'rid' (integer) and 'solo_st' (integer: 0 or 1).")]
        #[tool(aggr)] args: SetTrackSoloArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_solo_tool for rid: {}, solo_state: {}", args.rid, args.solo_st);

        if !(args.solo_st == 0 || args.solo_st == 1) {
            tracing::warn!("Invalid solo_st value: {}. Must be 0 or 1.", args.solo_st);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid solo_st value: {}. Must be 0 (off) or 1 (on).",
                args.solo_st
            ))]));
        }

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Int(args.solo_st)];
        let address = "/ardour/routes/solo";
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} solo state set to {}",
                args.rid, args.solo_st
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_track_rec_enable", description = "Sets the record enable state of a specific track.")]
    async fn set_track_rec_enable_tool(
        &self,
        #[schemars(description = "Arguments for setting track record enable state. Requires 'rid' (integer) and 'rec_st' (integer: 0 or 1).")]
        #[tool(aggr)] args: SetTrackRecEnableArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_rec_enable_tool for rid: {}, rec_enable_state: {}", args.rid, args.rec_st);

        if !(args.rec_st == 0 || args.rec_st == 1) {
            tracing::warn!("Invalid rec_st value: {}. Must be 0 or 1.", args.rec_st);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid rec_st value: {}. Must be 0 (off) or 1 (on).",
                args.rec_st
            ))]));
        }

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Int(args.rec_st)];
        let address = "/ardour/routes/recenable";
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} record enable state set to {}",
                args.rid, args.rec_st
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_track_gain_abs", description = "Sets the absolute gain of a specific track.")]
    async fn set_track_gain_abs_tool(
        &self,
        #[schemars(description = "Arguments for setting track absolute gain. Requires 'rid' (integer) and 'gain_abs' (float: 0.0 to 2.0).")]
        #[tool(aggr)] args: SetTrackGainAbsArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_gain_abs_tool for rid: {}, gain_abs: {}", args.rid, args.gain_abs);

        if !(args.gain_abs >= 0.0 && args.gain_abs <= 2.0) {
            tracing::warn!("Invalid gain_abs value: {}. Must be between 0.0 and 2.0.", args.gain_abs);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid gain_abs value: {}. Must be between 0.0 and 2.0.",
                args.gain_abs
            ))]));
        }

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Float(args.gain_abs)];
        let address = "/ardour/routes/gainabs";
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} absolute gain set to {}",
                args.rid, args.gain_abs
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_track_gain_db", description = "Sets the gain of a specific track in dB.")]
    async fn set_track_gain_db_tool(
        &self,
        #[schemars(description = "Arguments for setting track gain in dB. Requires 'rid' (integer) and 'gain_db' (float: -400.0 to 6.0).")]
        #[tool(aggr)] args: SetTrackGainDBArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_gain_db_tool for rid: {}, gain_db: {}", args.rid, args.gain_db);

        if !(args.gain_db >= -400.0 && args.gain_db <= 6.0) {
            tracing::warn!("Invalid gain_db value: {}. Must be between -400.0 and 6.0.", args.gain_db);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid gain_db value: {}. Must be between -400.0 and 6.0.",
                args.gain_db
            ))]));
        }

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Float(args.gain_db)];
        let address = "/ardour/routes/gaindB";
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} gain (dB) set to {}",
                args.rid, args.gain_db
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_track_trim_abs", description = "Sets the absolute trim of a specific track.")]
    async fn set_track_trim_abs_tool(
        &self,
        #[schemars(description = "Arguments for setting track absolute trim. Requires 'rid' (integer) and 'trim_abs' (float: 0.1 to 10.0).")]
        #[tool(aggr)] args: SetTrackTrimAbsArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_trim_abs_tool for rid: {}, trim_abs: {}", args.rid, args.trim_abs);

        if !(args.trim_abs >= 0.1 && args.trim_abs <= 10.0) {
            tracing::warn!("Invalid trim_abs value: {}. Must be between 0.1 and 10.0.", args.trim_abs);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid trim_abs value: {}. Must be between 0.1 and 10.0.",
                args.trim_abs
            ))]));
        }

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Float(args.trim_abs)];
        let address = "/ardour/routes/trimabs";
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} absolute trim set to {}",
                args.rid, args.trim_abs
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_track_trim_db", description = "Sets the trim of a specific track in dB.")]
    async fn set_track_trim_db_tool(
        &self,
        #[schemars(description = "Arguments for setting track trim in dB. Requires 'rid' (integer) and 'trim_db' (float: -20.0 to 20.0).")]
        #[tool(aggr)] args: SetTrackTrimDBArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_trim_db_tool for rid: {}, trim_db: {}", args.rid, args.trim_db);

        if args.trim_db < -20.0 || args.trim_db > 20.0 {
            let error_msg = format!(
                "Invalid trim_db value: {}. Must be between -20.0 and 20.0.",
                args.trim_db
            );
            tracing::warn!("{}", error_msg);
            return Ok(CallToolResult::error(vec![Content::text(error_msg)]));
        }

        let osc_addr = "/ardour/routes/trimdB";
        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Float(args.trim_db)];

        match self.send_osc_message(osc_addr, Some(osc_args)).await {
            Ok(_) => {
                let success_msg = format!(
                    "Successfully sent OSC message {} with rid {} and trim_db {}",
                    osc_addr, args.rid, args.trim_db
                );
                tracing::info!("{}", success_msg);
                Ok(CallToolResult::success(vec![Content::text(success_msg)]))
            }
            Err(e) => {
                let error_msg = format!("Failed to send OSC message {} for rid {}: {:?}", osc_addr, args.rid, e);
                tracing::error!("{}", error_msg);
                Err(McpError::internal_error(error_msg, None))
            }
        }
    }

    #[tool(name = "access_action", description = "Executes a specified Ardour menu action by its name.")]
    async fn access_action_tool(
        &self,
        #[schemars(description = "Argument for accessing menu action. Requires 'action_name' (string).")]
        #[tool(aggr)] args: AccessActionArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing access_action_tool for action_name: {}", args.action_name);

        if args.action_name.is_empty() {
            let error_msg = "action_name cannot be empty.";
            tracing::warn!("{}", error_msg);
            return Ok(CallToolResult::error(vec![Content::text(error_msg.to_string())]));
        }

        let osc_addr = "/ardour/access_action";
        // According to Ardour OSC docs, the action name is sent as a string argument
        let osc_args = vec![osc::Type::String(args.action_name.clone())];

        match self.send_osc_message(osc_addr, Some(osc_args)).await {
            Ok(_) => {
                let success_msg = format!(
                    "Successfully sent OSC message {} with action_name '{}'",
                    osc_addr, args.action_name
                );
                tracing::info!("{}", success_msg);
                Ok(CallToolResult::success(vec![Content::text(success_msg)]))
            }
            Err(e) => {
                let error_msg = format!(
                    "Failed to send OSC message {} for action_name '{}': {:?}",
                    osc_addr, args.action_name, e
                );
                tracing::error!("{}", error_msg);
                Err(McpError::internal_error(error_msg, None))
            }
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for ArdourService {
    fn get_info(&self) -> ServerInfo {
        let ardour_implementation = Implementation {
            name: "Ardour MCP Server (Rust)".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let server_capabilities = ServerCapabilities::builder()
            .enable_tools()
            .build();

        ServerInfo {
            server_info: ardour_implementation,
            capabilities: server_capabilities,
            instructions: Some("This server controls Ardour DAW transport functions.".to_string()),
            protocol_version: ProtocolVersion::V_2024_11_05,
        }
    }

    async fn list_resources(
        &self,
        _request: PaginatedRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let mut raw_playback_state_resource = RawResource::new(
            "ardour:/state/playback",
            "Ardour Playback State"
        );
        raw_playback_state_resource.description = Some(
            "Current playback state of Ardour (e.g., Playing, Stopped, Unknown).".to_string()
        );

        let playback_state_resource: Resource = raw_playback_state_resource.no_annotation();
        
        Ok(ListResourcesResult { 
            resources: vec![playback_state_resource],
            next_cursor: None,
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let resource_uri = request.uri.as_str();
        match resource_uri {
            "ardour:/state/playback" => {
                let state = self.ardour_state.lock().await;
                let status_str = match state.playback_status {
                    PlaybackStatus::Playing => "Playing",
                    PlaybackStatus::Stopped => "Stopped",
                    PlaybackStatus::Unknown => "Unknown",
                };
                Ok(ReadResourceResult {
                    contents: vec![rmcp::model::ResourceContents::TextResourceContents {
                        uri: resource_uri.to_string(),
                        mime_type: Some("text/plain".to_string()),
                        text: status_str.to_string(),
                    }],
                })
            }
            "ardour:/strip/list" => {
                let state = self.ardour_state.lock().await;
                // Filter out placeholder entries that might not have been filled if SSIDs are sparse
                // and our resize_with created default entries with id=0.
                let valid_strips: Vec<&TrackInfo> = state.strip_list.iter().filter(|ti| ti.id != 0).collect();
                
                match serde_json::to_string_pretty(&valid_strips) {
                    Ok(json_response) => Ok(ReadResourceResult {
                        contents: vec![rmcp::model::ResourceContents::TextResourceContents {
                            uri: resource_uri.to_string(),
                            mime_type: Some("application/json".to_string()),
                            text: json_response,
                        }],
                    }),
                    Err(e) => {
                        tracing::error!("Failed to serialize strip list: {}", e);
                        Err(McpError::internal_error(
                            format!("Failed to serialize strip list: {}", e),
                            None
                        ))
                    }
                }
            }
            "ardour:/action/list" => {
                let placeholder_actions = json!([
                    { "name": "Session/Save", "description": "Saves the current session." },
                    { "name": "Editor/zoom-to-session", "description": "Zooms to fit the entire session." },
                    { "name": "Transport/Loop", "description": "Toggles loop playback." }
                ]);
                Ok(ReadResourceResult {
                    contents: vec![rmcp::model::ResourceContents::TextResourceContents {
                        uri: resource_uri.to_string(),
                        mime_type: Some("application/json".to_string()),
                        text: placeholder_actions.to_string(),
                    }],
                })
            }
            _ => {
                // For not_found, rmcp expects an McpError with ErrorCode::ResourceNotFound
                // We should return an McpError, not Ok(ReadResourceResult)
                Err(McpError::resource_not_found(
                    format!("Resource URI '{}' not found.", resource_uri),
                    Some(json!({ "uri": resource_uri }))
                ))
            }
        }
    }

    async fn list_prompts(
        &self,
        _request: PaginatedRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
        Ok(ListPromptsResult { 
            prompts: vec![],
            next_cursor: None,
        })
    }

    async fn get_prompt(
        &self,
        _req: GetPromptRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, McpError> {
        Err(McpError::invalid_params("Prompt not found", None))
    }
}

// OSC Feedback Listener
async fn run_osc_feedback_listener(feedback_port: u16) {
    let listen_addr_str = format!("0.0.0.0:{}", feedback_port);
    info!(
        "Ardour OSC feedback listener starting on {} for incoming messages from Ardour.",
        listen_addr_str
    );

    let mut receiver = match osc::Receiver::bind(listen_addr_str.as_str()) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to bind OSC receiver for feedback on {}: {}. Feedback will not be processed.", listen_addr_str, e);
            return;
        }
    };
    info!("Successfully bound OSC feedback listener to {}", listen_addr_str);

    loop {
        match receiver.recv().await {
            Ok(Some((packet, src_addr))) => {
                // Process the received packet
                for msg in packet.into_msgs() {
                    info!(
                        "OSC Feedback Received from {}: Addr: '{}', Args: {:?}",
                        src_addr, msg.addr, msg.args
                    );
                    // Further processing can be added here (e.g., updating shared state)
                }
            }
            Ok(None) => {
                // This case can occur if the receiver is used in a non-blocking way
                // or if it's gracefully shutting down. For a continuously running server,
                // this might indicate an issue or an unexpected state.
                // For now, we can log it if it becomes frequent or problematic.
                // info!("OSC feedback receiver got None. This is unexpected in blocking mode.");
            }
            Err(e) => {
                warn!("Error receiving OSC feedback: {}. Listener will continue.", e);
                // Depending on the error, we might want to re-bind or implement backoff.
                // For now, log and continue to avoid crashing the listener on transient network issues.
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::load_from_env();

    // Configure tracing subscriber
    let log_level_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let ardour_log_file_path = settings
        .ardour_mcp_log_file
        .clone()
        .unwrap_or_else(|| "ardour_mcp_server.log".to_string());
    let file_appender = tracing_appender::rolling::daily(".", ardour_log_file_path);
    let (non_blocking_file_writer, _guard_file) = tracing_appender::non_blocking(file_appender);

    let stderr_make_writer = std::io::stderr.with_max_level(tracing::Level::INFO);
    let file_make_writer = non_blocking_file_writer.with_max_level(tracing::Level::DEBUG);

    // Combine stderr and file writer using Tee
    let combined_writer = Tee::new(stderr_make_writer, file_make_writer);

    tracing_subscriber::fmt()
        .with_env_filter(log_level_filter)
        .with_writer(combined_writer)
        .init_with_filter_ext();

    info!("Ardour MCP Server starting...");
    info!(
        "Attempting to send commands to Ardour OSC at: {}:{}",
        settings.ardour_osc_host, settings.ardour_osc_port_remote
    );
    info!(
        "Will listen for Ardour OSC feedback on: 0.0.0.0:{}",
        settings.ardour_osc_port_feedback // Log the feedback port
    );

    let ardour_service = Arc::new(ArdourService::new(settings.clone()).await);

    // Spawn the OSC feedback listener task
    let feedback_listener_settings = settings.clone(); // Clone settings for the listener task
    tokio::spawn(async move {
        run_osc_feedback_listener(feedback_listener_settings.ardour_osc_port_feedback).await;
    });

    let mut router = Router::new(ardour_service.clone());

    // Register tools - ensure this is done before starting the service
    // Transport Tools
    tools::register_all_tools(&mut router);
    resources::register_all_resources(&mut router);

    let service = Service::new(router, TypedAdvertise::default_tcp_advertise());

    service.serve().await?;
    info!("Ardour MCP Server has shut down.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::RawContent;

    fn setup_test_service() -> ArdourService {
        ArdourService::new().expect("Failed to create ArdourService for test")
    }

    #[test]
    fn test_get_server_info() {
        let service = setup_test_service();
        let info = service.get_info();

        assert_eq!(info.server_info.name, "Ardour MCP Server (Rust)");
        assert_eq!(info.server_info.version, env!("CARGO_PKG_VERSION").to_string());
        
        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
        assert!(info.capabilities.tools.is_some(), "Tools capability should be Some");
        assert_eq!(info.instructions, Some("This server controls Ardour DAW transport functions.".to_string()));
    }

    #[tokio::test]
    async fn test_transport_play_tool_reports_success_or_osc_error() {
        let service = setup_test_service();
        let result = service.transport_play_tool().await;

        assert!(result.is_ok(), "transport_play_tool call itself failed (should return Ok(CallToolResult) even on OSC error): {:?}", result.err());
        if let Ok(call_result) = result {
            if call_result.is_error == Some(true) {
                tracing::warn!("transport_play_tool reported an OSC error (this is acceptable for the test if Ardour is not perfectly reachable): {:?}", call_result.content);
                assert!(!call_result.content.is_empty(), "Error CallToolResult should have content explaining the error.");
            } else {
                assert!(!call_result.content.is_empty(), "Successful CallToolResult.content was empty");
                let contents = &call_result.content;
                assert_eq!(contents.len(), 1, "Expected one content item for success");
                let content_item = contents.get(0).expect("Failed to get content item for success");
                match &content_item.raw {
                    RawContent::Text(text_val) => {
                        assert_eq!(text_val.text, "Playback started");
                    }
                    other => {
                        panic!("Expected RawContent::Text for success, got {:?}", other);
                    }
                }
            }
        }
    }

    #[tokio::test]
    async fn test_set_track_mute_tool_arg_parsing() {
        let service = setup_test_service(); 

        let args_struct = SetTrackMuteArgs { rid: 1, mute_state: true };
        let result = service.set_track_mute_tool(args_struct).await;
        assert!(result.is_ok());
        if let Ok(call_result) = result {
             if call_result.is_error == Some(true) {
                tracing::warn!("set_track_mute_tool reported an OSC error, args might be fine but send failed: {:?}", call_result.content);
            } else {
                assert!(!call_result.content.is_empty(), "Successful mute should have descriptive content.");
            }
        }
    }

     #[tokio::test]
    async fn test_set_transport_speed_tool_arg_validation() {
        let service = setup_test_service();

        let args_out_of_range = SetTransportSpeedArgs { speed: 10.0 }; 
        let result_oor = service.set_transport_speed_tool(args_out_of_range).await;
        assert!(result_oor.is_err(), "Expected an McpError for out-of-range speed, but got Ok");

        let args_in_range = SetTransportSpeedArgs { speed: 1.5 };
        let result_ir = service.set_transport_speed_tool(args_in_range).await;
        assert!(result_ir.is_ok(), "Expected Ok(CallToolResult) for in-range speed, but got McpError");
         if let Ok(call_result) = result_ir {
             if call_result.is_error == Some(true) {
                tracing::warn!("set_transport_speed_tool (in range) reported an OSC error, args were fine but send failed: {:?}", call_result.content);
            } else {
                assert!(!call_result.content.is_empty(), "Successful speed set should have descriptive content.");
            }
        }
    }
} 