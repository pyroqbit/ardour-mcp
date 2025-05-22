use anyhow::Result;
use rmcp::{
    model::{
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
        ToolsCapability,
        ResourcesCapability,
    },
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

use ardour_mcp::TrackInfo;

const ARDOUR_OSC_TARGET_ADDR: &str = "127.0.0.1:3819";
#[allow(dead_code)]
const MCP_SERVER_OSC_LISTEN_ADDR: &str = "127.0.0.1:9099";

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
    transport_frame: Option<i64>, // Current playhead position in samples
}

impl ArdourState {
    fn new() -> Self {
        Self {
            playback_status: PlaybackStatus::Unknown,
            strip_list: Vec::new(),
            transport_frame: None,
        }
    }
}

#[derive(Clone)]
struct ArdourService {
    osc_sender: Arc<Mutex<osc::Sender<osc::Connected>>>,
    ardour_state: Arc<Mutex<ArdourState>>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTrackMuteArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired mute state (true for mute, false for unmute).")]
    mute_state: bool,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTransportSpeedArgs {
    #[schemars(description = "The desired transport speed. Valid range: -8.0 to 8.0.")]
    speed: f32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct LocateToolArgs {
    #[schemars(description = "The position in samples to locate to.")]
    spos: i64, 
    #[schemars(description = "Whether to start playing after locating. 0 for stop, 1 for play.")]
    roll: i32, 
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTrackSoloArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired solo state. 0 for solo off, 1 for solo on.")]
    solo_st: i32, 
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTrackRecEnableArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired record enable state. 0 for off, 1 for on.")]
    rec_st: i32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTrackGainAbsArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired absolute gain. Valid range: 0.0 to 2.0.")]
    gain_abs: f32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTrackGainDBArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired gain in dB. Valid range: -400.0 to 6.0.")]
    gain_db: f32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTrackTrimAbsArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired absolute trim. Valid range: 0.1 to 10.0.")]
    trim_abs: f32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetTrackTrimDBArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The desired trim in dB. Valid range: -20.0 to 20.0.")]
    trim_db: f32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct AccessActionArgs {
    #[schemars(description = "The name of the Ardour menu action to execute (e.g., 'Editor/zoom-to-session').")]
    action_name: String,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SelectStripArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus to select.")]
    rid: i32,
    #[schemars(description = "The desired select state (true to select). Currently, only true (1) is effective for selection.")]
    select_state: bool,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetStripPluginActiveArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The 1-indexed slot of the plugin on the strip.")]
    plugin_slot: i32, 
    #[schemars(description = "The desired activation state (true for active, false for inactive).")]
    active_state: bool,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetStripPluginParameterArgs {
    #[schemars(description = "The Router ID (rid) of the track/bus.")]
    rid: i32,
    #[schemars(description = "The 1-indexed slot of the plugin on the strip.")]
    plugin_slot: i32,
    #[schemars(description = "The 1-indexed ID of the parameter within the plugin.")]
    param_id: i32,
    #[schemars(description = "The desired parameter value, normalized (0.0 to 1.0).")]
    value: f32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetStripPanStereoWidthArgs {
    #[schemars(description = "The Router ID (rid) of the stereo track/bus.")]
    rid: i32,
    #[schemars(description = "The desired stereo width. Valid range: 0.0 to 1.0. Default is 1.0 (full width).")]
    width: f32,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[allow(dead_code)]
struct SetSelectedStripPanStereoWidthArgs {
    #[schemars(description = "The desired stereo width for the currently selected strip. Valid range: 0.0 to 1.0. Default is 1.0 (full width).")]
    width: f32,
}

#[tool(tool_box)] 
impl ArdourService {
    pub fn new() -> Result<Self> { 
        tracing::info!("Attempting to create OSC sender for Ardour at {}", ARDOUR_OSC_TARGET_ADDR);
        let sender = osc::sender()
            .map_err(|e| anyhow::anyhow!("Failed to create OSC sender builder: {}", e))?
            .connect(ARDOUR_OSC_TARGET_ADDR)
            .map_err(|e| anyhow::anyhow!("Failed to prepare OSC sender for {}: {}", ARDOUR_OSC_TARGET_ADDR, e))?;
        tracing::info!("OSC sender created and connected to Ardour at {}", ARDOUR_OSC_TARGET_ADDR);
        Ok(Self {
            osc_sender: Arc::new(Mutex::new(sender)),
            ardour_state: Arc::new(Mutex::new(ArdourState::new())),
        })
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

    #[tool(name = "transport_play", description = "Starts Ardour playback.")]
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
        
        match self.send_osc_message("/strip/mute", Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} mute state set to {}",
                args.rid, args.mute_state
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for /strip/mute: {}",
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
        match self.send_osc_message("/locate", Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Located to sample {} with roll state {}",
                args.spos, args.roll
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for /locate: {}",
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
        let address = "/strip/solo"; 
        
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
        let address = "/strip/recenable"; 
        
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
        #[schemars(description = "Arguments for setting track absolute gain. Requires 'rid' (integer) and 'gain_abs' (float: 0.0 to 2.0). Maps to fader 0.0-1.0.")]
        #[tool(aggr)] args: SetTrackGainAbsArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_gain_abs_tool for rid: {}, gain_abs: {}", args.rid, args.gain_abs);

        
        if args.gain_abs < 0.0 || args.gain_abs > 2.0 {
            tracing::warn!("Invalid gain_abs value: {}. Must be between 0.0 and 2.0 (maps to fader 0.0-1.0).", args.gain_abs);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid gain_abs value: {}. Must be between 0.0 and 2.0 (maps to fader 0.0-1.0).",
                args.gain_abs
            ))]));
        }
        let fader_position = (args.gain_abs / 2.0).clamp(0.0, 1.0);

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Float(fader_position)];
        let address = "/strip/fader"; 
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} fader position set to {} (from gain_abs {})",
                args.rid, fader_position, args.gain_abs
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
        let address = "/strip/gain"; 
        
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
        #[schemars(description = "Arguments for setting track absolute trim. Requires 'rid' (integer) and 'trim_abs' (float: 0.1 to 10.0). Maps to trim_fader 0.0-1.0.")]
        #[tool(aggr)] args: SetTrackTrimAbsArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing set_track_trim_abs_tool for rid: {}, trim_abs: {}", args.rid, args.trim_abs);

        
        if args.trim_abs < 0.1 || args.trim_abs > 10.0 {
            tracing::warn!("Invalid trim_abs value: {}. Must be between 0.1 and 10.0.", args.trim_abs);
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid trim_abs value: {}. Must be between 0.1 and 10.0.",
                args.trim_abs
            ))]));
        }
        let fader_position = ((args.trim_abs - 0.1) / 9.9).clamp(0.0, 1.0);

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Float(fader_position)];
        let address = "/strip/trim_fader"; 
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Track {} trim fader position set to {} (from trim_abs {})",
                args.rid, fader_position, args.trim_abs
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

        let osc_addr = "/strip/trimdB"; 
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

        let osc_addr = "/access_action"; 
        
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

    #[tool(name = "select_strip", description = "Selects a specific strip (track/bus) in Ardour.")]
    async fn select_strip_tool(
        &self,
        #[schemars(description = "Arguments for selecting a strip. Requires 'rid' (integer) and 'select_state' (boolean, true to select).")]
        #[tool(aggr)] args: SelectStripArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!("Executing select_strip_tool for rid: {}, select_state: {}", args.rid, args.select_state);

        // According to Ardour OSC docs for /strip/select, the second arg (y/n) is 1 for select, and 0 is ignored.
        // So, we only send if select_state is true.
        if !args.select_state {
            return Ok(CallToolResult::success(vec![Content::text(format!(
                "Strip {} not selected as select_state was false.",
                args.rid
            ))]));
        }

        let osc_args = vec![osc::Type::Int(args.rid), osc::Type::Int(1)]; // Always send 1 to select
        let address = "/strip/select"; 
        
        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Strip {} selection command sent.",
                args.rid
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_strip_plugin_active", description = "Activates or deactivates a plugin on a specific strip slot.")]
    async fn set_strip_plugin_active_tool(
        &self,
        #[schemars(description = "Arguments for setting plugin activation state. Requires 'rid' (strip ID), 'plugin_slot' (1-indexed), and 'active_state' (boolean).")]
        #[tool(aggr)] args: SetStripPluginActiveArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Executing set_strip_plugin_active_tool for rid: {}, plugin_slot: {}, active_state: {}",
            args.rid, args.plugin_slot, args.active_state
        );

        if args.plugin_slot <= 0 {
            return Ok(CallToolResult::error(vec![Content::text(
                "Invalid plugin_slot: must be a positive integer.".to_string()
            )]));
        }

        let address = if args.active_state {
            "/strip/plugin/activate"
        } else {
            "/strip/plugin/deactivate"
        };

        let osc_args = vec![
            osc::Type::Int(args.rid),
            osc::Type::Int(args.plugin_slot),
        ];

        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Strip {} plugin in slot {} command ({}) sent.",
                args.rid, args.plugin_slot, if args.active_state { "activate" } else { "deactivate" }
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_strip_plugin_parameter", description = "Sets a specific parameter of a plugin on a strip.")]
    async fn set_strip_plugin_parameter_tool(
        &self,
        #[schemars(description = "Arguments for setting a plugin parameter. Requires 'rid', 'plugin_slot', 'param_id', and 'value' (0.0-1.0).")]
        #[tool(aggr)] args: SetStripPluginParameterArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Executing set_strip_plugin_parameter_tool for rid: {}, plugin_slot: {}, param_id: {}, value: {}",
            args.rid, args.plugin_slot, args.param_id, args.value
        );

        if args.plugin_slot <= 0 {
            return Ok(CallToolResult::error(vec![Content::text(
                "Invalid plugin_slot: must be a positive integer.".to_string()
            )]));
        }
        if args.param_id <= 0 {
            return Ok(CallToolResult::error(vec![Content::text(
                "Invalid param_id: must be a positive integer.".to_string()
            )]));
        }
        if !(0.0..=1.0).contains(&args.value) {
            return Ok(CallToolResult::error(vec![Content::text(
                "Invalid value: must be between 0.0 and 1.0 (inclusive).".to_string()
            )]));
        }

        let osc_args = vec![
            osc::Type::Int(args.rid),
            osc::Type::Int(args.plugin_slot),
            osc::Type::Int(args.param_id),
            osc::Type::Float(args.value),
        ];
        let address = "/strip/plugin/parameter";

        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Strip {} plugin slot {} parameter {} set to {}.",
                args.rid, args.plugin_slot, args.param_id, args.value
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_strip_pan_stereo_width", description = "Sets the stereo width for a panner on a stereo strip.")]
    async fn set_strip_pan_stereo_width_tool(
        &self,
        #[schemars(description = "Arguments for setting stereo panner width. Requires 'rid' and 'width' (0.0-1.0).")]
        #[tool(aggr)] args: SetStripPanStereoWidthArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Executing set_strip_pan_stereo_width_tool for rid: {}, width: {}",
            args.rid, args.width
        );

        if !(0.0..=1.0).contains(&args.width) {
            return Ok(CallToolResult::error(vec![Content::text(
                "Invalid width: must be between 0.0 and 1.0 (inclusive).".to_string()
            )]));
        }

        let osc_args = vec![
            osc::Type::Int(args.rid),
            osc::Type::Float(args.width),
        ];
        let address = "/strip/panner/width";

        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Strip {} stereo pan width set to {}.",
                args.rid, args.width
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }

    #[tool(name = "set_selected_strip_pan_stereo_width", description = "Sets the stereo width for the currently selected stereo strip's panner.")]
    async fn set_selected_strip_pan_stereo_width_tool(
        &self,
        #[schemars(description = "Arguments for setting selected strip stereo panner width. Requires 'width' (0.0-1.0).")]
        #[tool(aggr)] args: SetSelectedStripPanStereoWidthArgs
    ) -> Result<CallToolResult, McpError> {
        tracing::info!(
            "Executing set_selected_strip_pan_stereo_width_tool with width: {}",
            args.width
        );

        if !(0.0..=1.0).contains(&args.width) {
            return Ok(CallToolResult::error(vec![Content::text(
                "Invalid width: must be between 0.0 and 1.0 (inclusive).".to_string()
            )]));
        }

        let osc_args = vec![
            osc::Type::Float(args.width),
        ];
        let address = "/select/pan_stereo_width";

        match self.send_osc_message(address, Some(osc_args)).await {
            Ok(_) => Ok(CallToolResult::success(vec![Content::text(format!(
                "Selected strip stereo pan width set to {}.
 (Ensure a strip was selected prior to calling this)",
                args.width
            ))])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                "OSC send error for {}: {}",
                address, e
            ))])),
        }
    }
}

impl ServerHandler for ArdourService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            server_info: Implementation {
                name: "ardour-mcp-server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability {
                    list_changed: Some(false),
                }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(false),
                    list_changed: Some(false),
                }),
                prompts: None,
                experimental: None,
                logging: None,
            },
            instructions: Some("Ardour MCP server for OSC control.".to_string()),
        }
    }

    async fn list_tools(
        &self,
        _request: PaginatedRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, McpError> {
        Ok(rmcp::model::ListToolsResult {
            next_cursor: None,
            tools: ArdourService::tool_box().list(),
        })
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParam,
        ctx: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::CallToolResult, McpError> {
        let tool_call_context = rmcp::handler::server::tool::ToolCallContext::new(self, request, ctx);
        ArdourService::tool_box().call(tool_call_context).await
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

        let mut raw_transport_frame_resource = RawResource::new(
            "ardour:/state/transport_frame",
            "Ardour Transport Frame Position"
        );
        raw_transport_frame_resource.description = Some(
            "Current playhead position in samples. Returns 'Unknown' if not yet reported by Ardour.".to_string()
        );
        let transport_frame_resource: Resource = raw_transport_frame_resource.no_annotation();
        
        let all_resources = vec![playback_state_resource, transport_frame_resource];

        tracing::debug!("Listing resources. Count: {}, Content: {:?}", all_resources.len(), all_resources);
        
        Ok(ListResourcesResult { 
            resources: all_resources,
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
            "ardour:/state/transport_frame" => {
                let state = self.ardour_state.lock().await;
                let frame_str = match state.transport_frame {
                    Some(frame) => frame.to_string(),
                    None => "Unknown".to_string(),
                };
                Ok(ReadResourceResult {
                    contents: vec![rmcp::model::ResourceContents::TextResourceContents {
                        uri: resource_uri.to_string(),
                        mime_type: Some("text/plain".to_string()),
                        text: frame_str,
                    }],
                })
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

#[tokio::main]
async fn main() -> Result<()> {
    // Create log directory if it doesn't exist
    let log_dir = Path::new("logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir)?;
    }
    let log_file_path = log_dir.join("ardour_mcp_server.log");

    // Create or append to the log file
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file_path)?;

    // Create a combined writer for file and stderr
    let stderr_writer = std::io::stderr.with_max_level(tracing::Level::INFO); // Log INFO and above to stderr
    let file_writer = log_file.with_max_level(tracing::Level::DEBUG); // Log DEBUG and above to file
    let combined_writer = stderr_writer.and(file_writer);

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(combined_writer) // Use the combined writer
        .with_ansi(true) // Enable ANSI for terminal, will be ignored by file
        .init();

    tracing::info!("\n======================================================================\nNEW SERVER RUN: {}\n======================================================================", chrono::Local::now().to_rfc2822());

    let ardour_service = ArdourService::new()?;
    
    // Send initial OSC setup to Ardour
    if let Err(e) = ardour_service.send_osc_setup_to_ardour().await {
        tracing::warn!("Could not send initial OSC setup to Ardour: {}. Feedback might not work.", e);
        // Decide if this should be a fatal error or just a warning. For now, warning.
    }

    let ardour_state_clone = Arc::clone(&ardour_service.ardour_state);
    
    let server_process = ardour_service.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("MCP Server serving error: {:?}", e);
    })?;

    tracing::info!("Ardour MCP server started and waiting for connections...");

    let _osc_listener_handle = tokio::spawn(async move {
        if let Err(e) = listen_ardour_osc_events(ardour_state_clone).await {
            tracing::error!("OSC listener task failed: {:?}", e);
        }
    });

    server_process.waiting().await?; 

    tracing::info!("Ardour MCP server stopped.");

    Ok(())
}

async fn listen_ardour_osc_events(state: Arc<Mutex<ArdourState>>) -> Result<()> { 
    tracing::info!(
        "Starting OSC listener for Ardour events on {}",
        MCP_SERVER_OSC_LISTEN_ADDR
    );

    let listen_socket = tokio::net::UdpSocket::bind(MCP_SERVER_OSC_LISTEN_ADDR)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind Tokio UDP socket for OSC on {}: {}", MCP_SERVER_OSC_LISTEN_ADDR, e))?;
    
    tracing::info!("Tokio UDP socket for OSC bound to {}", MCP_SERVER_OSC_LISTEN_ADDR);
    
    let mut buf = [0u8; osc::recv::DEFAULT_MTU];

    loop {
        match listen_socket.recv_from(&mut buf).await {
            Ok((size, peer_addr)) => {
                let packet = osc::decode(&buf[..size])
                    .map_err(|e| anyhow::anyhow!("OSC decode error from {}: {}", peer_addr, e))?;
                handle_osc_packet(packet, peer_addr, Arc::clone(&state)).await;
            }
            Err(e) => {
                // Log non-fatal errors, but break on others
                tracing::error!("OSC recv_from error: {}. Listener might stop.", e);
                // Consider if we should break or continue on certain errors.
                // For now, let's break on any error to avoid tight loops on persistent issues.
                break Err(anyhow::anyhow!("OSC recv_from error: {}", e)); 
            }
        }
    }
}

async fn handle_osc_packet(packet: osc::Packet, peer_addr: std::net::SocketAddr, state: Arc<Mutex<ArdourState>>) {
    match packet {
        osc::Packet::Message(msg) => {
            tracing::debug!("Received OSC message from {}: {} {:?}", peer_addr, msg.addr, msg.args);
            
            // Handle /strip/name/<ssid> <name>
            if msg.addr.starts_with("/strip/name/") {
                let parts: Vec<&str> = msg.addr.split('/').collect();
                if parts.len() == 4 { // expecting "", "strip", "name", "<ssid>"
                    if let Ok(ssid) = parts[3].parse::<i32>() {
                        if ssid > 0 { // SSIDs are 1-indexed
                            if let Some(osc::Type::String(name)) = msg.args.get(0) {
                                tracing::info!("Ardour feedback: /strip/name/{} -> {}", ssid, name);
            let mut current_state = state.lock().await;
                                let vec_idx = (ssid - 1) as usize;

                                // Ensure strip_list is large enough
                                if vec_idx >= current_state.strip_list.len() {
                                    current_state.strip_list.resize_with(vec_idx + 1, || TrackInfo {
                                        id: 0, // Will be overwritten by actual ssid if this is the target new strip
                                        name: String::new(),
                                        track_type: "unknown".to_string(),
                                    });
                                }
                                
                                // Update or fill the slot
                                let strip_info = &mut current_state.strip_list[vec_idx];
                                strip_info.id = ssid; // Set/confirm the ID
                                strip_info.name = name.clone();
                                // track_type will be updated if/when we get a /strip/type message
                                tracing::debug!("Updated strip_list: SSID {}, Name '{}', Type '{}'", strip_info.id, strip_info.name, strip_info.track_type);
                            }
                        }
                    }
                }
            } 
            // Handle /strip/type/<ssid> <type_str_or_int> (Hypothetical, based on common patterns)
            else if msg.addr.starts_with("/strip/type/") { // Note the 'else if'
                let parts: Vec<&str> = msg.addr.split('/').collect();
                if parts.len() == 4 { // expecting "", "strip", "type", "<ssid>"
                    if let Ok(ssid) = parts[3].parse::<i32>() {
                        if ssid > 0 { 
                            if let Some(type_arg) = msg.args.get(0) {
                                let type_str = match type_arg {
                                    osc::Type::String(s) => s.clone(),
                                    osc::Type::Int(i) => format!("type_id_{}", i), // Or map to known string types
                                    _ => "unknown_type_format".to_string(),
                                };
                                tracing::info!("Ardour feedback: /strip/type/{} -> {}", ssid, type_str);
                                let mut current_state = state.lock().await;
                                let vec_idx = (ssid - 1) as usize;

                                if vec_idx < current_state.strip_list.len() {
                                    // Only update if strip already known from a /name message (or pre-sized)
                                    let strip_info = &mut current_state.strip_list[vec_idx];
                                    if strip_info.id == ssid { // Ensure it's the correct strip, not a placeholder from resize
                                        strip_info.track_type = type_str;
                                        tracing::debug!("Updated strip_list: SSID {}, Name '{}', Type '{}'", strip_info.id, strip_info.name, strip_info.track_type);
                                    } else {
                                        tracing::warn!("Received /strip/type/{} but strip_list[{}] has id {}, expected {}. Type not updated.", ssid, vec_idx, strip_info.id, ssid);
                                    }
                                } else {
                                    tracing::warn!("Received /strip/type/{} but strip {} is out of bounds for current strip_list (len {}). Type not updated.", ssid, ssid, current_state.strip_list.len());
                                    // Optionally, could create a new entry here if /type can come before /name
                                    // current_state.strip_list.resize_with(vec_idx + 1, || TrackInfo { id: 0, name: String::new(), track_type: "unknown".to_string() });
                                    // current_state.strip_list[vec_idx].id = ssid;
                                    // current_state.strip_list[vec_idx].track_type = type_str;
                                }
                            }
                        }
                    }
                }
            }
            // Handle /transport_state <state_int> <speed_float> (no /ardour/ prefix, new arg order)
            else if msg.addr == "/transport_state" { 
                if msg.args.len() == 2 { // Expecting state and speed
                    let transport_state_val = msg.args.get(0).and_then(|arg| if let osc::Type::Int(s) = arg { Some(*s) } else { None });
                    let speed = msg.args.get(1).and_then(|arg| if let osc::Type::Float(s) = arg { Some(*s) } else { None });

                    if let (Some(ts_val), Some(s)) = (transport_state_val, speed) {
                        tracing::info!("Ardour feedback: /transport_state state: {}, speed: {}", ts_val, s);
                        let mut current_state_guard = state.lock().await;
                        match ts_val {
                            0 => { // Stopped
                                current_state_guard.playback_status = PlaybackStatus::Stopped;
                                tracing::info!("Playback status updated to Stopped via /transport_state");
                            }
                            1 => { // Rolling (Playing)
                                current_state_guard.playback_status = PlaybackStatus::Playing;
                                tracing::info!("Playback status updated to Playing (Rolling) via /transport_state");
                            }
                            2 => { // Looping (also Playing)
                                current_state_guard.playback_status = PlaybackStatus::Playing;
                                tracing::info!("Playback status updated to Playing (Looping) via /transport_state");
                            }
                            _ => {
                                tracing::warn!("Received /transport_state with unknown state value: {}", ts_val);
                            }
                        }
                    } else {
                        tracing::warn!("Received /transport_state with unexpected argument types: {:?}. Expected Int, Float.", msg.args);
                    }
                } else {
                    tracing::warn!("Received /transport_state with incorrect number of arguments: {}. Expected 2.", msg.args.len());
                }
            }
            // Handle /transport_frame <frame_int64> (no /ardour/ prefix)
            else if msg.addr == "/transport_frame" { // Changed from "/ardour/transport_frame"
                if msg.args.len() == 1 {
                    if let Some(osc::Type::Long(frame)) = msg.args.get(0) {
                        tracing::info!("Ardour feedback: /transport_frame -> {}", frame);
                        let mut current_state_guard = state.lock().await;
                        current_state_guard.transport_frame = Some(*frame);
                    } else {
                        tracing::warn!("Received /transport_frame with unexpected argument type: {:?}. Expected Long.", msg.args.get(0));
                    }
                } else {
                    tracing::warn!("Received /transport_frame with incorrect number of arguments: {}. Expected 1.", msg.args.len());
                }
            }
            // Handle playback status updates from /strip/play (original logic)
            else if msg.addr.as_str() == "/strip/play" { // Note the 'else if'
                let mut current_state = state.lock().await; // Moved lock inside specific message handling
                    if let Some(osc::Type::Int(is_playing_val)) = msg.args.get(0) {
                        if *is_playing_val == 1 {
                            current_state.playback_status = PlaybackStatus::Playing;
                            tracing::info!("Ardour feedback via /strip/play: Playback Started (state=1)");
                        } else if *is_playing_val == 0 {
                            current_state.playback_status = PlaybackStatus::Stopped;
                            tracing::info!("Ardour feedback via /strip/play: Playback Stopped (state=0)");
                        } else {
                            tracing::warn!(
                                "Ardour feedback via /strip/play: Received with unexpected integer state: {}. Ignoring for playback status.",
                                is_playing_val
                            );
                        }
                    } else {
                        tracing::warn!(
                            "Ardour feedback via /strip/play: Received without expected integer argument. Args: {:?}. Ignoring for playback status.",
                            msg.args
                        );
                    }
                }
            // Catch-all for other /strip/ messages for debugging
            else if msg.addr.starts_with("/strip/") { // Note the 'else if'
                    tracing::debug!("Received other Ardour /strip/ feedback: {} {:?}", msg.addr, msg.args);
                }
            // Generic unhandled message log (optional)
            // else {
                    // tracing::debug!("Received unhandled OSC message: {} {:?}", msg.addr, msg.args);
            // }
        }
        osc::Packet::Bundle(bundle) => {
            tracing::debug!("Received OSC bundle from {}:", peer_addr);
            for p_in_bundle in bundle.content {
                Box::pin(handle_osc_packet(p_in_bundle.into(), peer_addr, Arc::clone(&state))).await;
            }
        }
    }
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