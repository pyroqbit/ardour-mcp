# Ardour OSC Command Implementation Status

**IMPORTANT NOTE (2025-05-15):** Ardour versions 5.x and newer have significantly changed their OSC path structure. Global commands (e.g., transport) generally do not use an `/ardour/` prefix (e.g., `/locate` instead of `/ardour/locate`). Track-specific commands typically use `/strip/...` (e.g., `/strip/mute`) instead of the older `/ardour/routes/...`. This list reflects these updates. Server tools have been updated to use these newer paths.

Status of implemented OSC commands.
All listed commands are confirmed working with Ardour via recent tests based on server logs and client tester interactions, or have been updated to reflect newer OSC documentation.

## Transport Control

- [x] `/transport_stop` (Implemented as `transport_stop_tool`)
- [x] `/transport_play` (Implemented as `transport_play_tool`)
- [x] `/set_transport_speed` (f: speed) (Implemented as `set_transport_speed_tool`)
- [x] `/ffwd` (Implemented as `ffwd_tool`)
- [x] `/rewind` (Implemented as `rewind_tool`)
- [x] `/goto_start` (Implemented as `goto_start_tool`)
- [x] `/goto_end` (Implemented as `goto_end_tool`)
- [x] `/add_marker` (Implemented as `add_marker_tool`)
- [x] `/next_marker` (Implemented as `next_marker_tool`)
- [x] `/prev_marker` (Implemented as `prev_marker_tool`)
- [x] `/locate` (spos: samples, roll: 0 or 1 for play) (Implemented as `locate_tool`)
- [x] `/loop_toggle` (Implemented as `loop_toggle_tool`)
- [x] `/rec_enable_toggle` (Implemented as `rec_enable_toggle_tool`)
- [x] `/click` (Implemented as `click_toggle_tool`) - *Path confirmed from Ardour 4.7 doc, may need verification for 5.x+*

## Editing Related

- [x] `/undo` (Implemented as `undo_tool`)
- [x] `/redo` (Implemented as `redo_tool`)
- [x] `/save_state` (Implemented as `save_state_tool`)

## Record Control

- [x] `/toggle_punch_in` (Implemented as `toggle_punch_in_tool`)
- [x] `/toggle_punch_out` (Implemented as `toggle_punch_out_tool`)
- [x] `/toggle_all_rec_enables` (Implemented as `toggle_all_rec_enables_tool`)

## Track specific operations
*(Paths updated to `/strip/...` based on newer Ardour OSC documentation.)*

- [x] `/strip/mute` (rid: i32, mute_state: bool -> i32 0/1) (Implemented as `set_track_mute_tool`)
- [x] `/strip/solo` (rid: i32, solo_st: i32 0/1) (Implemented as `set_track_solo_tool`)
- [x] `/strip/recenable` (rid: i32, rec_st: i32 0/1) (Implemented as `set_track_rec_enable_tool`)
- [x] `/strip/fader` (rid: i32, position: f32 0.0-1.0) (Implemented as `set_track_gain_abs_tool`, input 0.0-2.0 normalized to 0.0-1.0)
- [x] `/strip/gain` (rid: i32, gain_db: f32 -400.0-6.0) (Implemented as `set_track_gain_db_tool`)
- [x] `/strip/trim_fader` (rid: i32, position: f32 0.0-1.0) (Implemented as `set_track_trim_abs_tool`, input 0.1-10.0 normalized to 0.0-1.0)
- [x] `/strip/trimdB` (rid: i32, trim_db: f32 -20.0-20.0) (Implemented as `set_track_trim_db_tool`)

## Menu Actions

- [x] `/access_action` (s: action_name) (Implemented as `access_action_tool`)
  - This is a powerful generic tool that allows triggering a wide variety of Ardour actions available through its menu system or action scripts.
  - The `action_name` corresponds to the internal name of the Ardour action.
  - **Examples of `action_name`**:
    - `Session/Save` (Saves the current session)
    - `Editor/zoom-to-session` (Zooms to fit the entire session in the editor view)
    - `Transport/Loop` (Toggles loop playback)
    - `Window/toggle-big-clock` (Shows/hides the big clock window)
    - `Region/normalize-region` (Normalizes the selected region(s) - requires a region to be selected)
    - `Track/add-audio-track` (Adds a new audio track)
    - `Mixer/toggle-show-mixer` (Shows/hides the mixer window)
  - You can discover action names by:
    1.  Exploring Ardour's menu structure (`.menu` files in Ardour's configuration or installation directory).
    2.  Using Ardour's Lua scripting interface to list actions.
    3.  Checking Ardour's official documentation or community resources for lists of common actions.
  - *Note: The success of an action may depend on the current state of Ardour (e.g., a region must be selected for region-specific actions). The server will send the command, but Ardour handles the execution and any resulting errors if preconditions are not met.*

## Feedback/State (Resource Reading)
*(OSC feedback paths in `handle_osc_packet` updated to remove `/ardour/` prefix where appropriate, e.g. `/transport_state`, `/transport_frame`)*

- [x] `ardour:/state/playback` (Implemented as a readable resource `playback_state_resource`)
  - Returns the current playback state (e.g., "Stopped", "Playing"). Updated by `/transport_state` and `/strip/play` feedback.
- [~] `ardour:/strip/list` (Partially implemented - Handles `/strip/name` and `/strip/type` feedback to build list)
  - **Intended functionality**: Lists available tracks and buses with their ID, name, and type.
  - **Current state**: Dynamically built from Ardour's `/strip/name` and `/strip/type` feedback.
- [x] `ardour:/state/transport_frame` (Implemented as `transport_frame_resource`)
  - Returns current playhead in samples. Updated by `/transport_frame` feedback.
- [~] `ardour:/action/list` (Partially implemented - Returns a placeholder JSON list of example actions)
  - **Intended functionality**: Lists available Ardour menu actions.
  - **Current state**: Returns a hardcoded example. Listing all actions dynamically is complex.


## Unimplemented Commands (from Ardour 4.7 OSC Doc - Paths may need verification for 5.x+)

*   [`/strip/select` (rid: i32, select_state: bool -> i32 0/1, 0 is ignored)](../src/main.rs#L666-L689) (Implemented as `select_strip_tool`, replaces `/ardour/select/route`)
*   `/ardour/select/sendroute` (i: RouteID, i: SendIX) - *Send selection*
*   `/ardour/select/insert` (i: RouteID, i: InsertIX) - *Insert/Plugin selection*
*   `/ardour/select/plugin` (i: RouteID, i: InsertIX) - *Synonymous with above*
*   `/ardour/select/auxsend` (i: RouteID, i: SendIX) - *Synonymous with sendroute selection*
*   `/ardour/select/clearall` - *Clear all selections*
*   `/ardour/routes/panners/xy` (RouteID, X, Y)
*   `/ardour/routes/panners/balance` (RouteID, pos)
*   `/ardour/routes/make_active` (RouteID)
*   `/ardour/routes/make_inactive` (RouteID)
*   `/ardour/sends/gainabs` (RouteID, SendIX, gain_abs)
*   `/ardour/sends/gainDB` (RouteID, SendIX, gain_dB)
*   `/ardour/inserts/bypass` (RouteID, InsertIX, bypass_state)
*   `/ardour/plugin/parameter` (RouteID, InsertIX, ParamIX, value) - *Generic plugin parameter control*
*   `/ardour/plugin/select` (RouteID, InsertIX) - *Likely for plugin editor focus*
*   `/ardour/select_strip_type` (i: StripTypeFlags) - *Control surface strip filtering*
*   `/ardour/select_send_page` (i: SendPage) - *Control surface send paging*
*   `/ardour/select_plugin_page` (i: PluginPage) - *Control surface plugin paging*
*   `/strip/monitor_input` (SSID, state) - *Newer OSC path for input monitoring*
*   `/strip/monitor_disk` (SSID, state) - *Newer OSC path for disk monitoring*
*   `/strip/solo_iso` (SSID, state) - *Solo Isolate*
*   `/strip/solo_safe` (SSID, state) - *Solo Safe*
*   `/strip/mute_dim` (SSID, state) - *Dim when muted*
*   `/strip/order_key` (SSID, key_string) - *Change strip order*
*   [`/strip/plugin/activate` (SSID, slot, active_state: bool)](../src/main.rs#L691-L719) (Implemented as `set_strip_plugin_active_tool` - sends `/strip/plugin/activate SSID slot` or `/strip/plugin/deactivate SSID slot`)
*   [`/strip/plugin/parameter` (SSID, slot, param_id, value: f32 0.0-1.0)](../src/main.rs#L721-L753) (Implemented as `set_strip_plugin_parameter_tool`)
*   [`/strip/panner/width` (SSID, width: f32 0.0-1.0)](../src/main.rs#L755-L778) (Implemented as `set_strip_pan_stereo_width_tool` - Note: uses generic panner path)
*   `/strip/pan_stereo_pan` (SSID, pan)
*   [`/select/pan_stereo_width` (width: f32 0.0-1.0)](../src/main.rs#L780-L805) (Implemented as `set_selected_strip_pan_stereo_width_tool` - Operates on selected strip)
*   Many more detailed strip controls in `osc58` doc. 