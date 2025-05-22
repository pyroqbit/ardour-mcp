# Ardour OSC Command List for MCP Server

This document lists the OSC commands that can be sent to Ardour via the MCP server.

## Implemented Commands

*   `/ardour/quit` - Quit Ardour - **Implemented**
*   `/ardour/save` - Save Session - **Implemented**
*   `/ardour/transport_stop` - Stop Transport - **Implemented**
*   `/ardour/transport_play` - Start Transport - **Implemented**
*   `/ardour/toggle_loop` - Toggle Loop Playback - **Implemented**
*   `/ardour/toggle_click` - Toggle Metronome Click - **Implemented**
*   `/ardour/goto_start` - Go to Start of Session - **Implemented**
*   `/ardour/goto_end` - Go to End of Session - **Implemented**
*   `/ardour/locate` - Locate playhead (spos: i64 samples, roll: i32 0/1) - **Implemented**
*   `/ardour/routes/solo` - Set track solo (rid: i32, solo_st: i32 0/1) - **Implemented**
*   `/ardour/routes/recenable` - Set track record enable (rid: i32, rec_st: i32 0/1) - **Implemented**
*   `/ardour/routes/gainabs` - Set track absolute gain (rid: i32, gain_abs: f32 0.0-2.0) - **Implemented**

## Not Implemented Commands (from Action List)

*   `/ardour/Common/Hide` - Hide - **Not Implemented**
*   `/ardour/Common/NewMIDITracer` - MIDI Tracer - **Not Implemented**
*   `/ardour/Common/ToggleMaximalEditor` - Maximise Editor Space - **Not Implemented**
*   `/ardour/Common/ToggleMaximalMixer` - Maximise Mixer Space - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack1` - Toggle Record Enable Track 1 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack10` - Toggle Record Enable Track 10 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack11` - Toggle Record Enable Track 11 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack12` - Toggle Record Enable Track 12 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack13` - Toggle Record Enable Track 13 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack14` - Toggle Record Enable Track 14 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack15` - Toggle Record Enable Track 15 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack16` - Toggle Record Enable Track 16 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack17` - Toggle Record Enable Track 17 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack18` - Toggle Record Enable Track 18 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack19` - Toggle Record Enable Track 19 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack2` - Toggle Record Enable Track 2 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack20` - Toggle Record Enable Track 20 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack21` - Toggle Record Enable Track 21 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack22` - Toggle Record Enable Track 22 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack23` - Toggle Record Enable Track 23 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack24` - Toggle Record Enable Track 24 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack25` - Toggle Record Enable Track 25 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack26` - Toggle Record Enable Track 26 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack27` - Toggle Record Enable Track 27 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack28` - Toggle Record Enable Track 28 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack29` - Toggle Record Enable Track 29 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack3` - Toggle Record Enable Track 3 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack30` - Toggle Record Enable Track 30 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack31` - Toggle Record Enable Track 31 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack32` - Toggle Record Enable Track 32 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack4` - Toggle Record Enable Track 4 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack5` - Toggle Record Enable Track 5 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack6` - Toggle Record Enable Track 6 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack7` - Toggle Record Enable Track 7 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack8` - Toggle Record Enable Track 8 - **Not Implemented**
*   `/ardour/Common/ToggleRecordEnableTrack9` - Toggle Record Enable Track 9 - **Not Implemented**
*   `/ardour/Common/add-location-from-playhead` - Add Mark from Playhead - **Not Implemented**
*   `/ardour/Common/addExistingAudioFiles` - Import - **Not Implemented**
*   `/ardour/Common/alt-finish-range` - Finish Range - **Not Implemented**
*   `/ardour/Common/alt-start-range` - Start Range - **Not Implemented**
*   `/ardour/Common/alternate-add-location-from-playhead` - Add Mark from Playhead - **Not Implemented**
*   `/ardour/Common/alternate-jump-backward-to-mark` - Jump to Previous Mark - **Not Implemented**
*   `/ardour/Common/alternate-jump-forward-to-mark` - Jump to Next Mark - **Not Implemented**
*   `/ardour/Common/alternate-remove-location-from-playhead` - Remove Mark at Playhead - **Not Implemented**
*   `/ardour/Common/attach-editor` - Attach - **Not Implemented**
*   `/ardour/Common/attach-mixer` - Attach - **Not Implemented**
*   `/ardour/Common/attach-preferences` - Attach - **Not Implemented**
*   `/ardour/Common/attach-recorder` - Attach - **Not Implemented**
*   `/ardour/Common/attach-trigger` - Attach - **Not Implemented**
*   `/ardour/Common/change-editor-visibility` - Change - **Not Implemented**
*   `/ardour/Common/change-mixer-visibility` - Change - **Not Implemented**
*   `/ardour/Common/change-preferences-visibility` - Change - **Not Implemented**
*   `/ardour/Common/change-recorder-visibility` - Change - **Not Implemented**
*   `/ardour/Common/change-trigger-visibility` - Change - **Not Implemented**
*   `/ardour/Common/chat` - Chat - **Not Implemented**
*   `/ardour/Common/deselect-all` - Deselect All - **Not Implemented**
*   `/ardour/Common/detach-editor` - Detach - **Not Implemented**
*   `/ardour/Common/detach-mixer` - Detach - **Not Implemented**
*   `/ardour/Common/detach-preferences` - Detach - **Not Implemented**
*   `/ardour/Common/detach-recorder` - Detach - **Not Implemented**
*   `/ardour/Common/detach-trigger` - Detach - **Not Implemented**
*   `/ardour/Common/finish-loop-range` - Finish Loop Range - **Not Implemented**
*   `/ardour/Common/finish-punch-range` - Finish Punch Range - **Not Implemented**
*   `/ardour/Common/finish-range` - Finish Range - **Not Implemented**
*   `/ardour/Common/finish-range-from-playhead` - Finish Range from Playhead - **Not Implemented**
*   `/ardour/Common/forums` - User Forums - **Not Implemented**
*   `/ardour/Common/hide-editor` - Hide - **Not Implemented**
*   `/ardour/Common/hide-mixer` - Hide - **Not Implemented**
*   `/ardour/Common/hide-preferences` - Hide - **Not Implemented**
*   `/ardour/Common/hide-recorder` - Hide - **Not Implemented**
*   `/ardour/Common/hide-trigger` - Hide - **Not Implemented**
*   `/ardour/Common/howto-report` - How to Report a Bug - **Not Implemented**
*   `/ardour/Common/invert-selection` - Invert Selection - **Not Implemented**
*   `/ardour/Common/jump-backward-to-mark` - Jump to Previous Mark - **Not Implemented**
*   `/ardour/Common/jump-forward-to-mark` - Jump to Next Mark - **Not Implemented**
*   `/ardour/Common/jump-to-loop-end` - Jump to Loop End - **Not Implemented**
*   `/ardour/Common/jump-to-loop-start` - Jump to Loop Start - **Not Implemented**
*   `/ardour/Common/key-change-editor-visibility` - Change - **Not Implemented**
*   `/ardour/Common/key-change-mixer-visibility` - Change - **Not Implemented**
*   `/ardour/Common/key-change-preferences-visibility` - Change - **Not Implemented**
*   `/ardour/Common/key-change-recorder-visibility` - Change - **Not Implemented**
*   `/ardour/Common/key-change-trigger-visibility` - Change - **Not Implemented**
*   `/ardour/Common/menu-show-preferences` - Preferences - **Not Implemented**
*   `/ardour/Common/next-tab` - Next Tab - **Not Implemented**
*   `/ardour/Common/nudge-next-backward` - Nudge Next Earlier - **Not Implemented**
*   `/ardour/Common/nudge-next-forward` - Nudge Next Later - **Not Implemented**
*   `/ardour/Common/nudge-playhead-backward` - Nudge Playhead Backward - **Not Implemented**
*   `/ardour/Common/nudge-playhead-forward` - Nudge Playhead Forward - **Not Implemented**
*   `/ardour/Common/playhead-backward-to-grid` - Playhead to Previous Grid - **Not Implemented**
*   `/ardour/Common/playhead-forward-to-grid` - Playhead to Next Grid - **Not Implemented**
*   `/ardour/Common/previous-tab` - Previous Tab - **Not Implemented**
*   `/ardour/Common/reference` - Reference - **Not Implemented**
*   `/ardour/Common/remove-location-from-playhead` - Remove Mark at Playhead - **Not Implemented**
*   `/ardour/Common/select-all-tracks` - Select All Tracks - **Not Implemented**
*   `/ardour/Common/select-all-visible-lanes` - Select All Visible Lanes - **Not Implemented**
*   `/ardour/Common/set-session-end-from-playhead` - Set Session End from Playhead - **Not Implemented**
*   `/ardour/Common/set-session-start-from-playhead` - Set Session Start from Playhead - **Not Implemented**
*   `/ardour/Common/show-editor` - Show Editor - **Not Implemented**
*   `/ardour/Common/show-mixer` - Show Mixer - **Not Implemented**
*   `/ardour/Common/show-preferences` - Show - **Not Implemented**
*   `/ardour/Common/show-recorder` - Show Recorder - **Not Implemented**
*   `/ardour/Common/show-trigger` - Show Cues - **Not Implemented**
*   `/ardour/Common/start-loop-range` - Start Loop Range - **Not Implemented**
*   `/ardour/Common/start-punch-range` - Start Punch Range - **Not Implemented**
*   `/ardour/Common/start-range` - Start Range - **Not Implemented**
*   `/ardour/Common/start-range-from-playhead` - Start Range from Playhead - **Not Implemented**
*   `/ardour/Common/toggle-editor-and-mixer` - Toggle Editor & Mixer - **Not Implemented**
*   `/ardour/Common/toggle-location-at-playhead` - Toggle Mark at Playhead - **Not Implemented**
*   `/ardour/Common/toggle-meterbridge` - Meterbridge - **Not Implemented**
*   `/ardour/Common/tracker` - Report a Bug - **Not Implemented**
*   `/ardour/Common/tutorial` - Tutorial - **Not Implemented**
*   `/ardour/Common/website` - Website - **Not Implemented**
*   `/ardour/Common/website-dev` - Development - **Not Implemented**
*   `/ardour/Cues/trigger-cue-0` - Trigger Cue A - **Not Implemented**
*   `/ardour/Cues/trigger-cue-1` - Trigger Cue B - **Not Implemented**
*   `/ardour/Cues/trigger-cue-10` - Trigger Cue K - **Not Implemented**
*   `/ardour/Cues/trigger-cue-11` - Trigger Cue L - **Not Implemented**
*   `/ardour/Cues/trigger-cue-12` - Trigger Cue M - **Not Implemented**
*   `/ardour/Cues/trigger-cue-13` - Trigger Cue N - **Not Implemented**
*   `/ardour/Cues/trigger-cue-14` - Trigger Cue O - **Not Implemented**
*   `/ardour/Cues/trigger-cue-15` - Trigger Cue P - **Not Implemented**
*   `/ardour/Cues/trigger-cue-2` - Trigger Cue C - **Not Implemented**
*   `/ardour/Cues/trigger-cue-3` - Trigger Cue D - **Not Implemented**
*   `/ardour/Cues/trigger-cue-4` - Trigger Cue E - **Not Implemented**
*   `/ardour/Cues/trigger-cue-5` - Trigger Cue F - **Not Implemented**
*   `/ardour/Cues/trigger-cue-6` - Trigger Cue G - **Not Implemented**
*   `/ardour/Cues/trigger-cue-7` - Trigger Cue H - **Not Implemented**
*   `/ardour/Cues/trigger-cue-8` - Trigger Cue I - **Not Implemented**
*   `/ardour/Cues/trigger-cue-9` - Trigger Cue J - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-1` - 1 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-10` - 10 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-11` - 11 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-12` - 12 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-13` - 13 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-14` - 14 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-15` - 15 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-16` - 16 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-2` - 2 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-3` - 3 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-4` - 4 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-5` - 5 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-6` - 6 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-7` - 7 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-8` - 8 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-9` - 9 - **Not Implemented**
*   `/ardour/DrawChannel/draw-channel-auto` - Auto - **Not Implemented**
*   `/ardour/DrawLength/draw-length-asixteenthbeat` - 1/64 Note - **Not Implemented**
*   `/ardour/DrawLength/draw-length-auto` - Auto - **Not Implemented**
*   `/ardour/DrawLength/draw-length-bar` - Bar - **Not Implemented**
*   `/ardour/DrawLength/draw-length-beat` - 1/4 Note - **Not Implemented**
*   `/ardour/DrawLength/draw-length-eighths` - 1/32 Note - **Not Implemented**
*   `/ardour/DrawLength/draw-length-fifths` - 1/5 (8th quintuplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-fourteenths` - 1/14 (16th septuplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-halves` - 1/8 Note - **Not Implemented**
*   `/ardour/DrawLength/draw-length-quarters` - 1/16 Note - **Not Implemented**
*   `/ardour/DrawLength/draw-length-sevenths` - 1/7 (8th septuplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-sixths` - 1/6 (16th triplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-tenths` - 1/10 (16th quintuplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-thirds` - 1/3 (8th triplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-thirtyseconds` - 1/128 Note - **Not Implemented**
*   `/ardour/DrawLength/draw-length-twelfths` - 1/12 (32nd triplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-twentieths` - 1/20 (32nd quintuplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-twentyeighths` - 1/28 (32nd septuplet) - **Not Implemented**
*   `/ardour/DrawLength/draw-length-twentyfourths` - 1/24 (64th triplet) - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-1` - 1 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-10` - 10 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-100` - 100 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-101` - 101 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-102` - 102 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-103` - 103 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-104` - 104 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-105` - 105 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-106` - 106 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-107` - 107 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-108` - 108 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-109` - 109 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-11` - 11 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-110` - 110 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-111` - 111 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-112` - 112 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-113` - 113 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-114` - 114 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-115` - 115 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-116` - 116 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-117` - 117 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-118` - 118 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-119` - 119 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-12` - 12 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-120` - 120 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-121` - 121 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-122` - 122 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-123` - 123 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-124` - 124 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-125` - 125 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-126` - 126 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-127` - 127 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-13` - 13 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-14` - 14 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-15` - 15 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-16` - 16 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-17` - 17 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-18` - 18 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-19` - 19 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-2` - 2 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-20` - 20 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-21` - 21 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-22` - 22 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-23` - 23 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-24` - 24 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-25` - 25 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-26` - 26 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-27` - 27 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-28` - 28 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-29` - 29 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-3` - 3 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-30` - 30 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-31` - 31 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-32` - 32 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-33` - 33 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-34` - 34 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-35` - 35 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-36` - 36 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-37` - 37 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-38` - 38 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-39` - 39 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-4` - 4 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-40` - 40 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-41` - 41 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-42` - 42 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-43` - 43 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-44` - 44 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-45` - 45 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-46` - 46 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-47` - 47 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-48` - 48 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-49` - 49 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-5` - 5 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-50` - 50 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-51` - 51 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-52` - 52 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-53` - 53 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-54` - 54 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-55` - 55 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-56` - 56 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-57` - 57 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-58` - 58 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-59` - 59 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-6` - 6 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-60` - 60 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-61` - 61 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-62` - 62 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-63` - 63 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-64` - 64 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-65` - 65 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-66` - 66 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-67` - 67 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-68` - 68 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-69` - 69 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-7` - 7 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-70` - 70 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-71` - 71 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-72` - 72 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-73` - 73 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-74` - 74 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-75` - 75 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-76` - 76 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-77` - 77 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-78` - 78 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-79` - 79 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-8` - 8 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-80` - 80 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-81` - 81 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-82` - 82 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-83` - 83 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-84` - 84 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-85` - 85 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-86` - 86 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-87` - 87 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-88` - 88 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-89` - 89 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-9` - 9 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-90` - 90 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-91` - 91 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-92` - 92 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-93` - 93 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-94` - 94 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-95` - 95 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-96` - 96 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-97` - 97 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-98` - 98 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-99` - 99 - **Not Implemented**
*   `/ardour/DrawVelocity/draw-velocity-auto` - Auto - **Not Implemented**
*   `/ardour/Editor/GridChoice` - Snap & Grid - **Not Implemented**
*   `/ardour/Editor/LoudnessAssistant` - Loudness Assistant... - **Not Implemented**
*   `/ardour/Editor/ToggleGroupTabs` - Show Group Tabs - **Not Implemented**
*   `/ardour/Editor/ToggleJadeo` - Video Monitor - **Not Implemented**
*   `/ardour/Editor/ToggleSummary` - Show Summary - **Not Implemented**
*   `/ardour/Editor/addExistingPTFiles` - Import PT session - **Not Implemented**
*   `/ardour/Editor/addExternalAudioToRegionList` - Import to Source List... - **Not Implemented**
*   `/ardour/Editor/alternate-alternate-redo` - Redo - **Not Implemented**
*   `/ardour/Editor/alternate-editor-delete` - Delete - **Not Implemented**
*   `/ardour/Editor/alternate-nudge-backward` - Nudge Earlier - **Not Implemented**
*   `/ardour/Editor/alternate-nudge-forward` - Nudge Later - **Not Implemented**
*   `/ardour/Editor/alternate-redo` - Redo - **Not Implemented**
*   `/ardour/Editor/alternate-select-all-after-edit-cursor` - Select All After Edit Point - **Not Implemented**
*   `/ardour/Editor/alternate-select-all-before-edit-cursor` - Select All Before Edit Point - **Not Implemented**
*   `/ardour/Editor/alternate-tab-to-transient-backwards` - Move to Previous Transient - **Not Implemented**
*   `/ardour/Editor/alternate-tab-to-transient-forwards` - Move to Next Transient - **Not Implemented**
*   `/ardour/Editor/bring-into-session` - Bring all media into session folder - **Not Implemented**
*   `/ardour/Editor/center-edit-cursor` - Center Edit Point - **Not Implemented**
*   `/ardour/Editor/center-playhead` - Center Playhead - **Not Implemented**
*   `/ardour/Editor/copy-playlists-for-all-tracks` - Copy Playlist For All Tracks - **Not Implemented**
*   `/ardour/Editor/copy-playlists-for-armed-tracks` - Copy Playlist For Rec-Armed Tracks - **Not Implemented**
*   `/ardour/Editor/copy-playlists-for-selected-tracks` - Copy Playlist For Selected Tracks - **Not Implemented**
*   `/ardour/Editor/crop` - Crop - **Not Implemented**
*   `/ardour/Editor/cycle-edit-mode` - Cycle Edit Mode - **Not Implemented**
*   `/ardour/Editor/cycle-edit-point` - Change Edit Point - **Not Implemented**
*   `/ardour/Editor/cycle-edit-point-with-marker` - Change Edit Point Including Marker - **Not Implemented**
*   `/ardour/Editor/cycle-snap-mode` - Toggle Snap - **Not Implemented**
*   `/ardour/Editor/cycle-zoom-focus` - Next Zoom Focus - **Not Implemented**
*   `/ardour/Editor/duplicate` - Duplicate - **Not Implemented**
*   `/ardour/Editor/edit-at-mouse` - Mouse - **Not Implemented**
*   `/ardour/Editor/edit-at-playhead` - Playhead - **Not Implemented**
*   `/ardour/Editor/edit-at-selected-marker` - Marker - **Not Implemented**
*   `/ardour/Editor/edit-current-meter` - Edit Current Time Signature - **Not Implemented**
*   `/ardour/Editor/edit-current-tempo` - Edit Current Tempo - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-next-region-end` - To Next Region End - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-next-region-start` - To Next Region Start - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-next-region-sync` - To Next Region Sync - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-previous-region-end` - To Previous Region End - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-previous-region-start` - To Previous Region Start - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-previous-region-sync` - To Previous Region Sync - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-range-end` - To Range End - **Not Implemented**
*   `/ardour/Editor/edit-cursor-to-range-start` - To Range Start - **Not Implemented**
*   `/ardour/Editor/edit-to-playhead` - Active Mark to Playhead - **Not Implemented**
*   `/ardour/Editor/editor-analyze-loudness` - Loudness Analysis - **Not Implemented**
*   `/ardour/Editor/editor-analyze-spectrum` - Spectral Analysis - **Not Implemented**
*   `/ardour/Editor/editor-consolidate` - Consolidate Range - **Not Implemented**
*   `/ardour/Editor/editor-consolidate-with-processing` - Consolidate Range (with processing) - **Not Implemented**
*   `/ardour/Editor/editor-copy` - Copy - **Not Implemented**
*   `/ardour/Editor/editor-crop` - Crop - **Not Implemented**
*   `/ardour/Editor/editor-cut` - Cut - **Not Implemented**
*   `/ardour/Editor/editor-delete` - Delete - **Not Implemented**
*   `/ardour/Editor/editor-fade-range` - Fade Range Selection - **Not Implemented**
*   `/ardour/Editor/editor-loudness-assistant` - Loudness Assistant - **Not Implemented**
*   `/ardour/Editor/editor-paste` - Paste - **Not Implemented**
*   `/ardour/Editor/editor-separate` - Separate - **Not Implemented**
*   `/ardour/Editor/expand-tracks` - Expand Track Height - **Not Implemented**
*   `/ardour/Editor/export-audio` - Export Audio - **Not Implemented**
*   `/ardour/Editor/export-range` - Export Range - **Not Implemented**
*   `/ardour/Editor/fit-selection` - Fit Selection (Vertical) - **Not Implemented**
*   `/ardour/Editor/fit_16_tracks` - Fit 16 Tracks - **Not Implemented**
*   `/ardour/Editor/fit_1_track` - Fit 1 Track - **Not Implemented**
*   `/ardour/Editor/fit_2_tracks` - Fit 2 Tracks - **Not Implemented**
*   `/ardour/Editor/fit_32_tracks` - Fit 32 Tracks - **Not Implemented**
*   `/ardour/Editor/fit_4_tracks` - Fit 4 Tracks - **Not Implemented**
*   `/ardour/Editor/fit_8_tracks` - Fit 8 Tracks - **Not Implemented**
*   `/ardour/Editor/fit_all_tracks` - Fit All Tracks - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-1` - Go to View 1 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-10` - Go to View 10 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-11` - Go to View 11 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-12` - Go to View 12 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-2` - Go to View 2 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-3` - Go to View 3 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-4` - Go to View 4 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-5` - Go to View 5 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-6` - Go to View 6 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-7` - Go to View 7 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-8` - Go to View 8 - **Not Implemented**
*   `/ardour/Editor/goto-visual-state-9` - Go to View 9 - **Not Implemented**
*   `/ardour/Editor/importFromSession` - Import from Session - **Not Implemented**
*   `/ardour/Editor/insert-time` - Insert Time - **Not Implemented**
*   `/ardour/Editor/layer-display-overlaid` - Overlaid layer display - **Not Implemented**
*   `/ardour/Editor/layer-display-stacked` - Stacked layer display - **Not Implemented**
*   `/ardour/Editor/lock` - Lock - **Not Implemented**
*   `/ardour/Editor/main-menu-play-selected-regions` - Play Selected Regions - **Not Implemented**
*   `/ardour/Editor/main-menu-tag-selected-regions` - Tag Selected Regions - **Not Implemented**
*   `/ardour/Editor/move-range-end-to-next-region-boundary` - Move Range End to Next Region Boundary - **Not Implemented**
*   `/ardour/Editor/move-range-end-to-previous-region-boundary` - Move Range End to Previous Region Boundary - **Not Implemented**
*   `/ardour/Editor/move-range-start-to-next-region-boundary` - Move Range Start to Next Region Boundary - **Not Implemented**
*   `/ardour/Editor/move-range-start-to-previous-region-boundary` - Move Range Start to Previous Region Boundary - **Not Implemented**
*   `/ardour/Editor/move-selected-tracks-down` - Move Selected Tracks Down - **Not Implemented**
*   `/ardour/Editor/move-selected-tracks-up` - Move Selected Tracks Up - **Not Implemented**
*   `/ardour/Editor/multi-duplicate` - Multi-Duplicate... - **Not Implemented**
*   `/ardour/Editor/new-playlists-for-all-tracks` - New Playlist For All Tracks - **Not Implemented**
*   `/ardour/Editor/new-playlists-for-armed-tracks` - New Playlist For Rec-Armed Tracks - **Not Implemented**
*   `/ardour/Editor/new-playlists-for-selected-tracks` - New Playlist For Selected Tracks - **Not Implemented**
*   `/ardour/Editor/next-grid-choice` - Next Quantize Grid Choice - **Not Implemented**
*   `/ardour/Editor/nudge-backward` - Nudge Earlier - **Not Implemented**
*   `/ardour/Editor/nudge-forward` - Nudge Later - **Not Implemented**
*   `/ardour/Editor/play-edit-range` - Play Edit Range - **Not Implemented**
*   `/ardour/Editor/play-from-edit-point` - Play from Edit Point - **Not Implemented**
*   `/ardour/Editor/play-from-edit-point-and-return` - Play from Edit Point and Return - **Not Implemented**
*   `/ardour/Editor/playhead-to-edit` - Playhead to Active Mark - **Not Implemented**
*   `/ardour/Editor/playhead-to-next-region-boundary` - Playhead to Next Region Boundary - **Not Implemented**
*   `/ardour/Editor/playhead-to-next-region-boundary-noselection` - Playhead to Next Region Boundary (No Track Selection) - **Not Implemented**
*   `/ardour/Editor/playhead-to-next-region-end` - Playhead to Next Region End - **Not Implemented**
*   `/ardour/Editor/playhead-to-next-region-start` - Playhead to Next Region Start - **Not Implemented**
*   `/ardour/Editor/playhead-to-next-region-sync` - Playhead to Next Region Sync - **Not Implemented**
*   `/ardour/Editor/playhead-to-previous-region-boundary` - Playhead to Previous Region Boundary - **Not Implemented**
*   `/ardour/Editor/playhead-to-previous-region-boundary-noselection` - Playhead to Previous Region Boundary (No Track Selection) - **Not Implemented**
*   `/ardour/Editor/playhead-to-previous-region-end` - Playhead to Previous Region End - **Not Implemented**
*   `/ardour/Editor/playhead-to-previous-region-start` - Playhead to Previous Region Start - **Not Implemented**
*   `/ardour/Editor/playhead-to-previous-region-sync` - Playhead to Previous Region Sync - **Not Implemented**
*   `/ardour/Editor/playhead-to-range-end` - Playhead to Range End - **Not Implemented**
*   `/ardour/Editor/playhead-to-range-start` - Playhead to Range Start - **Not Implemented**
*   `/ardour/Editor/prev-grid-choice` - Previous Quantize Grid Choice - **Not Implemented**
*   `/ardour/Editor/quantize` - Quantize - **Not Implemented**
*   `/ardour/Editor/redo` - Redo - **Not Implemented**
*   `/ardour/Editor/redo-last-selection-op` - Redo Selection Change - **Not Implemented**
*   `/ardour/Editor/remove-gaps` - Remove Gaps - **Not Implemented**
*   `/ardour/Editor/remove-last-capture` - Remove Last Capture - **Not Implemented**
*   `/ardour/Editor/remove-time` - Remove Time - **Not Implemented**
*   `/ardour/Editor/remove-track` - Remove - **Not Implemented**
*   `/ardour/Editor/save-visual-state-1` - Save View 1 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-10` - Save View 10 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-11` - Save View 11 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-12` - Save View 12 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-2` - Save View 2 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-3` - Save View 3 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-4` - Save View 4 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-5` - Save View 5 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-6` - Save View 6 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-7` - Save View 7 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-8` - Save View 8 - **Not Implemented**
*   `/ardour/Editor/save-visual-state-9` - Save View 9 - **Not Implemented**
*   `/ardour/Editor/scroll-backward` - Scroll Backward - **Not Implemented**
*   `/ardour/Editor/scroll-forward` - Scroll Forward - **Not Implemented**
*   `/ardour/Editor/scroll-playhead-backward` - Playhead Backward - **Not Implemented**
*   `/ardour/Editor/scroll-playhead-forward` - Playhead Forward - **Not Implemented**
*   `/ardour/Editor/scroll-tracks-down` - Scroll Tracks Down - **Not Implemented**
*   `/ardour/Editor/scroll-tracks-up` - Scroll Tracks Up - **Not Implemented**
*   `/ardour/Editor/select-all-after-edit-cursor` - Select All After Edit Point - **Not Implemented**
*   `/ardour/Editor/select-all-before-edit-cursor` - Select All Before Edit Point - **Not Implemented**
*   `/ardour/Editor/select-all-between-cursors` - Select All Overlapping Edit Range - **Not Implemented**
*   `/ardour/Editor/select-all-in-loop-range` - Select All in Loop Range - **Not Implemented**
*   `/ardour/Editor/select-all-in-punch-range` - Select All in Punch Range - **Not Implemented**
*   `/ardour/Editor/select-all-objects` - Select All Objects - **Not Implemented**
*   `/ardour/Editor/select-all-within-cursors` - Select All Inside Edit Range - **Not Implemented**
*   `/ardour/Editor/select-from-regions` - Set Range to Selected Regions - **Not Implemented**
*   `/ardour/Editor/select-loop-range` - Set Range to Loop Range - **Not Implemented**
*   `/ardour/Editor/select-next-route` - Select Next Track or Bus - **Not Implemented**
*   `/ardour/Editor/select-next-stripable` - Select Next Strip - **Not Implemented**
*   `/ardour/Editor/select-prev-route` - Select Previous Track or Bus - **Not Implemented**
*   `/ardour/Editor/select-prev-stripable` - Select Previous Strip - **Not Implemented**
*   `/ardour/Editor/select-punch-range` - Set Range to Punch Range - **Not Implemented**
*   `/ardour/Editor/select-range-between-cursors` - Select Edit Range - **Not Implemented**
*   `/ardour/Editor/select-topmost` - Select Topmost Track - **Not Implemented**
*   `/ardour/Editor/selected-marker-to-next-region-boundary` - To Next Region Boundary - **Not Implemented**
*   `/ardour/Editor/selected-marker-to-next-region-boundary-noselection` - To Next Region Boundary (No Track Selection) - **Not Implemented**
*   `/ardour/Editor/selected-marker-to-previous-region-boundary` - To Previous Region Boundary - **Not Implemented**
*   `/ardour/Editor/selected-marker-to-previous-region-boundary-noselection` - To Previous Region Boundary (No Track Selection) - **Not Implemented**
*   `/ardour/Editor/separate-from-loop` - Separate Using Loop Range - **Not Implemented**
*   `/ardour/Editor/separate-from-punch` - Separate Using Punch Range - **Not Implemented**
*   `/ardour/Editor/set-auto-punch-range` - Set Auto Punch In/Out from Playhead - **Not Implemented**
*   `/ardour/Editor/set-edit-lock` - Lock - **Not Implemented**
*   `/ardour/Editor/set-edit-point` - Active Marker to Mouse - **Not Implemented**
*   `/ardour/Editor/set-edit-ripple` - Ripple - **Not Implemented**
*   `/ardour/Editor/set-edit-slide` - Slide - **Not Implemented**
*   `/ardour/Editor/set-loop-from-edit-range` - Set Loop from Selection - **Not Implemented**
*   `/ardour/Editor/set-playhead` - Playhead to Mouse - **Not Implemented**
*   `/ardour/Editor/set-punch-from-edit-range` - Set Punch from Selection - **Not Implemented**
*   `/ardour/Editor/set-ripple-all` - All - **Not Implemented**
*   `/ardour/Editor/set-ripple-interview` - Interview - **Not Implemented**
*   `/ardour/Editor/set-ripple-selected` - Selected - **Not Implemented**
*   `/ardour/Editor/set-session-from-edit-range` - Set Session Start/End from Selection - **Not Implemented**
*   `/ardour/Editor/set-tempo-from-edit-range` - Set Tempo from Edit Range = Bar - **Not Implemented**
*   `/ardour/Editor/show-editor-list` - Show Editor List - **Not Implemented**
*   `/ardour/Editor/show-editor-mixer` - Show Editor Mixer - **Not Implemented**
*   `/ardour/Editor/show-marker-lines` - Show Marker Lines - **Not Implemented**
*   `/ardour/Editor/show-plist-selector` - Show Playlist Selector - **Not Implemented**
*   `/ardour/Editor/show-touched-automation` - Show Automation Lane on Touch - **Not Implemented**
*   `/ardour/Editor/shrink-tracks` - Shrink Track Height - **Not Implemented**
*   `/ardour/Editor/snap-magnetic` - Magnetic - **Not Implemented**
*   `/ardour/Editor/snap-normal` - Grid - **Not Implemented**
*   `/ardour/Editor/snap-off` - No Grid - **Not Implemented**
*   `/ardour/Editor/sound-midi-notes` - Sound Selected MIDI Notes - **Not Implemented**
*   `/ardour/Editor/split-region` - Split/Separate - **Not Implemented**
*   `/ardour/Editor/step-mouse-mode` - Step Mouse Mode - **Not Implemented**
*   `/ardour/Editor/step-tracks-down` - Step Tracks Down - **Not Implemented**
*   `/ardour/Editor/step-tracks-up` - Step Tracks Up - **Not Implemented**
*   `/ardour/Editor/tab-to-transient-backwards` - Move to Previous Transient - **Not Implemented**
*   `/ardour/Editor/tab-to-transient-forwards` - Move to Next Transient - **Not Implemented**
*   `/ardour/Editor/tag-last-capture` - Tag Last Capture - **Not Implemented**
*   `/ardour/Editor/temporal-zoom-in` - Zoom In - **Not Implemented**
*   `/ardour/Editor/temporal-zoom-out` - Zoom Out - **Not Implemented**
*   `/ardour/Editor/toggle-all-existing-automation` - Toggle All Existing Automation - **Not Implemented**
*   `/ardour/Editor/toggle-follow-playhead` - Follow Playhead - **Not Implemented**
*   `/ardour/Editor/toggle-layer-display` - Toggle Layer Display - **Not Implemented**
*   `/ardour/Editor/toggle-log-window` - Log - **Not Implemented**
*   `/ardour/Editor/toggle-midi-input-active` - Toggle MIDI Input Active for Editor-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Editor/toggle-skip-playback` - Use Skip Ranges - **Not Implemented**
*   `/ardour/Editor/toggle-stationary-playhead` - Stationary Playhead - **Not Implemented**
*   `/ardour/Editor/toggle-track-active` - Toggle Active - **Not Implemented**
*   `/ardour/Editor/toggle-vmon-frame` - Frame number - **Not Implemented**
*   `/ardour/Editor/toggle-vmon-fullscreen` - Fullscreen - **Not Implemented**
*   `/ardour/Editor/toggle-vmon-letterbox` - Letterbox - **Not Implemented**
*   `/ardour/Editor/toggle-vmon-ontop` - Always on Top - **Not Implemented**
*   `/ardour/Editor/toggle-vmon-osdbg` - Timecode Background - **Not Implemented**
*   `/ardour/Editor/toggle-vmon-timecode` - Timecode - **Not Implemented**
*   `/ardour/Editor/toggle-zoom` - Toggle Zoom State - **Not Implemented**
*   `/ardour/Editor/track-height-large` - Large - **Not Implemented**
*   `/ardour/Editor/track-height-larger` - Larger - **Not Implemented**
*   `/ardour/Editor/track-height-largest` - Largest - **Not Implemented**
*   `/ardour/Editor/track-height-normal` - Normal - **Not Implemented**
*   `/ardour/Editor/track-height-small` - Small - **Not Implemented**
*   `/ardour/Editor/track-mute-toggle` - Toggle Mute - **Not Implemented**
*   `/ardour/Editor/track-record-enable-toggle` - Toggle Record Enable - **Not Implemented**
*   `/ardour/Editor/track-solo-isolate-toggle` - Toggle Solo Isolate - **Not Implemented**
*   `/ardour/Editor/track-solo-toggle` - Toggle Solo - **Not Implemented**
*   `/ardour/Editor/undo` - Undo - **Not Implemented**
*   `/ardour/Editor/undo-last-selection-op` - Undo Selection Change - **Not Implemented**
*   `/ardour/Editor/zoom-to-extents` - Zoom to Extents - **Not Implemented**
*   `/ardour/Editor/zoom-to-selection` - Zoom to Selection - **Not Implemented**
*   `/ardour/Editor/zoom-to-selection-horiz` - Zoom to Selection (Horizontal) - **Not Implemented**
*   `/ardour/Editor/zoom-to-session` - Zoom to Session - **Not Implemented**
*   `/ardour/Editor/zoom-vmon-100` - Original Size - **Not Implemented**
*   `/ardour/Editor/zoom_100_ms` - Zoom to 100 ms - **Not Implemented**
*   `/ardour/Editor/zoom_10_min` - Zoom to 10 min - **Not Implemented**
*   `/ardour/Editor/zoom_10_ms` - Zoom to 10 ms - **Not Implemented**
*   `/ardour/Editor/zoom_10_sec` - Zoom to 10 sec - **Not Implemented**
*   `/ardour/Editor/zoom_1_min` - Zoom to 1 min - **Not Implemented**
*   `/ardour/Editor/zoom_1_sec` - Zoom to 1 sec - **Not Implemented**
*   `/ardour/Editor/zoom_5_min` - Zoom to 5 min - **Not Implemented**
*   `/ardour/EditorMenu/AlignMenu` - Align - **Not Implemented**
*   `/ardour/EditorMenu/AnalyzeMenu` - Analyze - **Not Implemented**
*   `/ardour/EditorMenu/Autoconnect` - Autoconnect - **Not Implemented**
*   `/ardour/EditorMenu/AutomationMenu` - Automation - **Not Implemented**
*   `/ardour/EditorMenu/ConsolidateMenu` - Consolidate - **Not Implemented**
*   `/ardour/EditorMenu/Crossfades` - Crossfades - **Not Implemented**
*   `/ardour/EditorMenu/CueMenu` - Cues - **Not Implemented**
*   `/ardour/EditorMenu/Edit` - Edit - **Not Implemented**
*   `/ardour/EditorMenu/EditCursorMovementOptions` - Move Selected Marker - **Not Implemented**
*   `/ardour/EditorMenu/EditPointMenu` - Edit Point - **Not Implemented**
*   `/ardour/EditorMenu/EditSelectRangeOptions` - Select Range Operations - **Not Implemented**
*   `/ardour/EditorMenu/EditSelectRegionOptions` - Select Regions - **Not Implemented**
*   `/ardour/EditorMenu/FadeMenu` - Fade - **Not Implemented**
*   `/ardour/EditorMenu/GridChoiceQuintuplets` - Quintuplets - **Not Implemented**
*   `/ardour/EditorMenu/GridChoiceSeptuplets` - Septuplets - **Not Implemented**
*   `/ardour/EditorMenu/GridChoiceTriplets` - Triplets - **Not Implemented**
*   `/ardour/EditorMenu/LatchMenu` - Latch - **Not Implemented**
*   `/ardour/EditorMenu/LayerDisplay` - Region Layers - **Not Implemented**
*   `/ardour/EditorMenu/Link` - Link - **Not Implemented**
*   `/ardour/EditorMenu/LocateToMarker` - Locate to Markers - **Not Implemented**
*   `/ardour/EditorMenu/LuaScripts` - Lua Scripts - **Not Implemented**
*   `/ardour/EditorMenu/MIDI` - MIDI Options - **Not Implemented**
*   `/ardour/EditorMenu/MarkerMenu` - Markers - **Not Implemented**
*   `/ardour/EditorMenu/MeterFalloff` - Meter falloff - **Not Implemented**
*   `/ardour/EditorMenu/MeterHold` - Meter hold - **Not Implemented**
*   `/ardour/EditorMenu/MiscOptions` - Misc Options - **Not Implemented**
*   `/ardour/EditorMenu/Monitoring` - Monitoring - **Not Implemented**
*   `/ardour/EditorMenu/MoveActiveMarkMenu` - Active Mark - **Not Implemented**
*   `/ardour/EditorMenu/MovePlayHeadMenu` - Playhead - **Not Implemented**
*   `/ardour/EditorMenu/PlayMenu` - Play - **Not Implemented**
*   `/ardour/EditorMenu/PrimaryClockMenu` - Primary Clock - **Not Implemented**
*   `/ardour/EditorMenu/Pullup` - Pullup / Pulldown - **Not Implemented**
*   `/ardour/EditorMenu/RegionEditOps` - Region operations - **Not Implemented**
*   `/ardour/EditorMenu/RegionGainMenu` - Gain - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenu` - Region - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuDuplicate` - Duplicate - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuEdit` - Edit - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuFades` - Fades - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuGain` - Gain - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuLayering` - Layering - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuMIDI` - MIDI - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuMarkers` - Markers - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuPosition` - Position - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuRanges` - Ranges - **Not Implemented**
*   `/ardour/EditorMenu/RegionMenuTrim` - Trim - **Not Implemented**
*   `/ardour/EditorMenu/RulerMenu` - Rulers - **Not Implemented**
*   `/ardour/EditorMenu/SavedViewMenu` - Editor Views - **Not Implemented**
*   `/ardour/EditorMenu/ScrollMenu` - Scroll - **Not Implemented**
*   `/ardour/EditorMenu/SecondaryClockMenu` - Secondary Clock - **Not Implemented**
*   `/ardour/EditorMenu/Select` - Select - **Not Implemented**
*   `/ardour/EditorMenu/SelectMenu` - Select - **Not Implemented**
*   `/ardour/EditorMenu/SeparateMenu` - Separate - **Not Implemented**
*   `/ardour/EditorMenu/SetLoopMenu` - Loop - **Not Implemented**
*   `/ardour/EditorMenu/SetPunchMenu` - Punch - **Not Implemented**
*   `/ardour/EditorMenu/Solo` - Solo - **Not Implemented**
*   `/ardour/EditorMenu/Subframes` - Subframes - **Not Implemented**
*   `/ardour/EditorMenu/SyncMenu` - Sync - **Not Implemented**
*   `/ardour/EditorMenu/TempoMenu` - Tempo - **Not Implemented**
*   `/ardour/EditorMenu/Timecode` - Timecode fps - **Not Implemented**
*   `/ardour/EditorMenu/Tools` - Tools - **Not Implemented**
*   `/ardour/EditorMenu/TrackHeightMenu` - Height - **Not Implemented**
*   `/ardour/EditorMenu/TrackMenu` - Track - **Not Implemented**
*   `/ardour/EditorMenu/TrackPlaylistMenu` - Playlists - **Not Implemented**
*   `/ardour/EditorMenu/VideoMonitorMenu` - Video Monitor - **Not Implemented**
*   `/ardour/EditorMenu/View` - View - **Not Implemented**
*   `/ardour/EditorMenu/ZoomFocus` - Zoom Focus - **Not Implemented**
*   `/ardour/EditorMenu/ZoomFocusMenu` - Zoom Focus - **Not Implemented**
*   `/ardour/EditorMenu/ZoomMenu` - Zoom - **Not Implemented**
*   `/ardour/LuaAction/script-1` - Unset #1 - **Not Implemented**
*   `/ardour/LuaAction/script-10` - Unset #10 - **Not Implemented**
*   `/ardour/LuaAction/script-11` - Unset #11 - **Not Implemented**
*   `/ardour/LuaAction/script-12` - Unset #12 - **Not Implemented**
*   `/ardour/LuaAction/script-13` - Unset #13 - **Not Implemented**
*   `/ardour/LuaAction/script-14` - Unset #14 - **Not Implemented**
*   `/ardour/LuaAction/script-15` - Unset #15 - **Not Implemented**
*   `/ardour/LuaAction/script-16` - Unset #16 - **Not Implemented**
*   `/ardour/LuaAction/script-17` - Unset #17 - **Not Implemented**
*   `/ardour/LuaAction/script-18` - Unset #18 - **Not Implemented**
*   `/ardour/LuaAction/script-19` - Unset #19 - **Not Implemented**
*   `/ardour/LuaAction/script-2` - Unset #2 - **Not Implemented**
*   `/ardour/LuaAction/script-20` - Unset #20 - **Not Implemented**
*   `/ardour/LuaAction/script-21` - Unset #21 - **Not Implemented**
*   `/ardour/LuaAction/script-22` - Unset #22 - **Not Implemented**
*   `/ardour/LuaAction/script-23` - Unset #23 - **Not Implemented**
*   `/ardour/LuaAction/script-24` - Unset #24 - **Not Implemented**
*   `/ardour/LuaAction/script-25` - Unset #25 - **Not Implemented**
*   `/ardour/LuaAction/script-26` - Unset #26 - **Not Implemented**
*   `/ardour/LuaAction/script-27` - Unset #27 - **Not Implemented**
*   `/ardour/LuaAction/script-28` - Unset #28 - **Not Implemented**
*   `/ardour/LuaAction/script-29` - Unset #29 - **Not Implemented**
*   `/ardour/LuaAction/script-3` - Unset #3 - **Not Implemented**
*   `/ardour/LuaAction/script-30` - Unset #30 - **Not Implemented**
*   `/ardour/LuaAction/script-31` - Unset #31 - **Not Implemented**
*   `/ardour/LuaAction/script-32` - Unset #32 - **Not Implemented**
*   `/ardour/LuaAction/script-4` - Unset #4 - **Not Implemented**
*   `/ardour/LuaAction/script-5` - Unset #5 - **Not Implemented**
*   `/ardour/LuaAction/script-6` - Unset #6 - **Not Implemented**
*   `/ardour/LuaAction/script-7` - Unset #7 - **Not Implemented**
*   `/ardour/LuaAction/script-8` - Unset #8 - **Not Implemented**
*   `/ardour/LuaAction/script-9` - Unset #9 - **Not Implemented**
*   `/ardour/MIDI/panic` - Panic (Send MIDI all-notes-off) - **Not Implemented**
*   `/ardour/Main Menu/AudioFileFormat` - Audio File Format - **Not Implemented**
*   `/ardour/Main Menu/AudioFileFormatData` - Sample Format - **Not Implemented**
*   `/ardour/Main Menu/AudioFileFormatHeader` - File Type - **Not Implemented**
*   `/ardour/Main Menu/Cleanup` - Clean-up - **Not Implemented**
*   `/ardour/Main Menu/ControlSurfaces` - Control Surfaces - **Not Implemented**
*   `/ardour/Main Menu/Denormals` - Denormal Handling - **Not Implemented**
*   `/ardour/Main Menu/DetachMenu` - Detach - **Not Implemented**
*   `/ardour/Main Menu/EditorMenu` - Editor - **Not Implemented**
*   `/ardour/Main Menu/Help` - Help - **Not Implemented**
*   `/ardour/Main Menu/KeyMouseActions` - Misc. Shortcuts - **Not Implemented**
*   `/ardour/Main Menu/Metering` - Metering - **Not Implemented**
*   `/ardour/Main Menu/MeteringFallOffRate` - Fall Off Rate - **Not Implemented**
*   `/ardour/Main Menu/MeteringHoldTime` - Hold Time - **Not Implemented**
*   `/ardour/Main Menu/MixerMenu` - Mixer - **Not Implemented**
*   `/ardour/Main Menu/Plugins` - Plugins - **Not Implemented**
*   `/ardour/Main Menu/PrefsMenu` - Preferences - **Not Implemented**
*   `/ardour/Main Menu/RecorderMenu` - Recorder - **Not Implemented**
*   `/ardour/Main Menu/Session` - Session - **Not Implemented**
*   `/ardour/Main Menu/Sync` - Sync - **Not Implemented**
*   `/ardour/Main Menu/TransportOptions` - Options - **Not Implemented**
*   `/ardour/Main Menu/TriggerMenu` - Cue Grid - **Not Implemented**
*   `/ardour/Main Menu/WindowMenu` - Window - **Not Implemented**
*   `/ardour/Main/AddTrackBus` - Add Track, Bus or VCA... - **Not Implemented**
*   `/ardour/Main/Archive` - Archive... - **Not Implemented**
*   `/ardour/Main/CleanupPeakFiles` - Rebuild Peak Files - **Not Implemented**
*   `/ardour/Main/CleanupUnusedRegions` - Clean-up Unused Regions... - **Not Implemented**
*   `/ardour/Main/CleanupUnusedSources` - Clean-up Unused Sources... - **Not Implemented**
*   `/ardour/Main/Close` - Close - **Not Implemented**
*   `/ardour/Main/CloseVideo` - Remove Video - **Not Implemented**
*   `/ardour/Main/EditMetadata` - Edit Metadata... - **Not Implemented**
*   `/ardour/Main/Escape` - Escape (deselect all) - **Not Implemented**
*   `/ardour/Main/Export` - Export - **Not Implemented**
*   `/ardour/Main/ExportAudio` - Export to Audio File(s)... - **Not Implemented**
*   `/ardour/Main/ExportVideo` - Export to Video File... - **Not Implemented**
*   `/ardour/Main/FlushWastebasket` - Flush Wastebasket - **Not Implemented**
*   `/ardour/Main/ImportMetadata` - Import Metadata... - **Not Implemented**
*   `/ardour/Main/ManageTemplates` - Templates - **Not Implemented**
*   `/ardour/Main/Metadata` - Metadata - **Not Implemented**
*   `/ardour/Main/MonitorMenu` - Monitor Section - **Not Implemented**
*   `/ardour/Main/New` - New... - **Not Implemented**
*   `/ardour/Main/Open` - Open... - **Not Implemented**
*   `/ardour/Main/OpenVideo` - Open Video... - **Not Implemented**
*   `/ardour/Main/QuickExport` - Quick Audio Export... - **Not Implemented**
*   `/ardour/Main/QuickSnapshotStay` - Quick Snapshot (& keep working on current version) ... - **Not Implemented**
*   `/ardour/Main/QuickSnapshotSwitch` - Quick Snapshot (& switch to new version) ... - **Not Implemented**
*   `/ardour/Main/Recent` - Recent... - **Not Implemented**
*   `/ardour/Main/Rename` - Rename... - **Not Implemented**
*   `/ardour/Main/SaveAs` - Save As... - **Not Implemented**
*   `/ardour/Main/SaveTemplate` - Save Template... - **Not Implemented**
*   `/ardour/Main/Scripting` - Scripting - **Not Implemented**
*   `/ardour/Main/SnapshotStay` - Snapshot (& keep working on current version) ... - **Not Implemented**
*   `/ardour/Main/SnapshotSwitch` - Snapshot (& switch to new version) ... - **Not Implemented**
*   `/ardour/Main/StemExport` - Stem export... - **Not Implemented**
*   `/ardour/Main/ToggleLatencyCompensation` - Disable Latency Compensation - **Not Implemented**
*   `/ardour/Main/cancel-solo` - Cancel Solo - **Not Implemented**
*   `/ardour/Main/close-current-dialog` - Close Current Dialog - **Not Implemented**
*   `/ardour/Main/duplicate-routes` - Duplicate Tracks/Busses... - **Not Implemented**
*   `/ardour/Mixer/ToggleFoldbackStrip` - Mixer: Show Foldback Strip - **Not Implemented**
*   `/ardour/Mixer/ToggleMixerList` - Mixer: Show Mixer List - **Not Implemented**
*   `/ardour/Mixer/ToggleMonitorSection` - Mixer: Show Monitor Section - **Not Implemented**
*   `/ardour/Mixer/ToggleVCAPane` - Mixer: Show VCAs - **Not Implemented**
*   `/ardour/Mixer/ab-plugins` - Toggle Selected Plugins - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-1` - Clear Mixer Scene #1 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-10` - Clear Mixer Scene #10 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-11` - Clear Mixer Scene #11 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-12` - Clear Mixer Scene #12 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-2` - Clear Mixer Scene #2 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-3` - Clear Mixer Scene #3 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-4` - Clear Mixer Scene #4 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-5` - Clear Mixer Scene #5 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-6` - Clear Mixer Scene #6 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-7` - Clear Mixer Scene #7 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-8` - Clear Mixer Scene #8 - **Not Implemented**
*   `/ardour/Mixer/clear-mixer-scene-9` - Clear Mixer Scene #9 - **Not Implemented**
*   `/ardour/Mixer/copy-processors` - Copy Selected Processors - **Not Implemented**
*   `/ardour/Mixer/cut-processors` - Cut Selected Processors - **Not Implemented**
*   `/ardour/Mixer/decrement-gain` - Increase Gain on Mixer-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Mixer/delete-processors` - Delete Selected Processors - **Not Implemented**
*   `/ardour/Mixer/increment-gain` - Decrease Gain on Mixer-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Mixer/mute` - Toggle Mute on Mixer-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Mixer/paste-processors` - Paste Selected Processors - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-1` - Recall Mixer Scene #1 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-10` - Recall Mixer Scene #10 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-11` - Recall Mixer Scene #11 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-12` - Recall Mixer Scene #12 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-2` - Recall Mixer Scene #2 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-3` - Recall Mixer Scene #3 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-4` - Recall Mixer Scene #4 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-5` - Recall Mixer Scene #5 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-6` - Recall Mixer Scene #6 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-7` - Recall Mixer Scene #7 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-8` - Recall Mixer Scene #8 - **Not Implemented**
*   `/ardour/Mixer/recall-mixer-scene-9` - Recall Mixer Scene #9 - **Not Implemented**
*   `/ardour/Mixer/recenable` - Toggle Rec-enable on Mixer-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Mixer/scroll-left` - Scroll Mixer Window to the left - **Not Implemented**
*   `/ardour/Mixer/scroll-right` - Scroll Mixer Window to the right - **Not Implemented**
*   `/ardour/Mixer/select-all-processors` - Select All (visible) Processors - **Not Implemented**
*   `/ardour/Mixer/select-next-stripable` - Select Next Mixer Strip - **Not Implemented**
*   `/ardour/Mixer/select-none` - Deselect all strips and processors - **Not Implemented**
*   `/ardour/Mixer/select-prev-stripable` - Select Previous Mixer Strip - **Not Implemented**
*   `/ardour/Mixer/solo` - Toggle Solo on Mixer-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-1` - Store Mixer Scene #1 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-10` - Store Mixer Scene #10 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-11` - Store Mixer Scene #11 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-12` - Store Mixer Scene #12 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-2` - Store Mixer Scene #2 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-3` - Store Mixer Scene #3 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-4` - Store Mixer Scene #4 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-5` - Store Mixer Scene #5 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-6` - Store Mixer Scene #6 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-7` - Store Mixer Scene #7 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-8` - Store Mixer Scene #8 - **Not Implemented**
*   `/ardour/Mixer/store-mixer-scene-9` - Store Mixer Scene #9 - **Not Implemented**
*   `/ardour/Mixer/toggle-disk-monitor` - Toggle Disk Monitoring - **Not Implemented**
*   `/ardour/Mixer/toggle-input-monitor` - Toggle Input Monitoring - **Not Implemented**
*   `/ardour/Mixer/toggle-midi-input-active` - Toggle MIDI Input Active for Mixer-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Mixer/toggle-processors` - Toggle Selected Processors - **Not Implemented**
*   `/ardour/Mixer/unity-gain` - Set Gain to 0dB on Mixer-Selected Tracks/Busses - **Not Implemented**
*   `/ardour/Monitor Section/monitor-cut-all` - Mute - **Not Implemented**
*   `/ardour/Monitor Section/monitor-dim-all` - Dim - **Not Implemented**
*   `/ardour/Monitor Section/monitor-mono` - Mono - **Not Implemented**
*   `/ardour/Monitor/UseMonitorSection` - Use Monitor Section - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-0` - Cut monitor channel 0 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-1` - Cut monitor channel 1 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-10` - Cut monitor channel 10 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-11` - Cut monitor channel 11 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-12` - Cut monitor channel 12 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-13` - Cut monitor channel 13 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-14` - Cut monitor channel 14 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-15` - Cut monitor channel 15 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-2` - Cut monitor channel 2 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-3` - Cut monitor channel 3 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-4` - Cut monitor channel 4 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-5` - Cut monitor channel 5 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-6` - Cut monitor channel 6 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-7` - Cut monitor channel 7 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-8` - Cut monitor channel 8 - **Not Implemented**
*   `/ardour/Monitor/monitor-cut-9` - Cut monitor channel 9 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-0` - Dim monitor channel 0 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-1` - Dim monitor channel 1 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-10` - Dim monitor channel 10 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-11` - Dim monitor channel 11 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-12` - Dim monitor channel 12 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-13` - Dim monitor channel 13 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-14` - Dim monitor channel 14 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-15` - Dim monitor channel 15 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-2` - Dim monitor channel 2 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-3` - Dim monitor channel 3 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-4` - Dim monitor channel 4 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-5` - Dim monitor channel 5 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-6` - Dim monitor channel 6 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-7` - Dim monitor channel 7 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-8` - Dim monitor channel 8 - **Not Implemented**
*   `/ardour/Monitor/monitor-dim-9` - Dim monitor channel 9 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-0` - Invert monitor channel 0 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-1` - Invert monitor channel 1 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-10` - Invert monitor channel 10 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-11` - Invert monitor channel 11 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-12` - Invert monitor channel 12 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-13` - Invert monitor channel 13 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-14` - Invert monitor channel 14 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-15` - Invert monitor channel 15 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-2` - Invert monitor channel 2 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-3` - Invert monitor channel 3 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-4` - Invert monitor channel 4 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-5` - Invert monitor channel 5 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-6` - Invert monitor channel 6 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-7` - Invert monitor channel 7 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-8` - Invert monitor channel 8 - **Not Implemented**
*   `/ardour/Monitor/monitor-invert-9` - Invert monitor channel 9 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-0` - Solo monitor channel 0 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-1` - Solo monitor channel 1 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-10` - Solo monitor channel 10 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-11` - Solo monitor channel 11 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-12` - Solo monitor channel 12 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-13` - Solo monitor channel 13 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-14` - Solo monitor channel 14 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-15` - Solo monitor channel 15 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-2` - Solo monitor channel 2 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-3` - Solo monitor channel 3 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-4` - Solo monitor channel 4 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-5` - Solo monitor channel 5 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-6` - Solo monitor channel 6 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-7` - Solo monitor channel 7 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-8` - Solo monitor channel 8 - **Not Implemented**
*   `/ardour/Monitor/monitor-solo-9` - Solo monitor channel 9 - **Not Implemented**
*   `/ardour/Monitor/toggle-monitor-processor-box` - Toggle Monitor Section Processor Box - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-audition` - Audition Tool - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-content` - Content Tool - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-cut` - Cut Tool - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-draw` - Note Drawing Tool - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-object` - Object Tool - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-object-range` - Smart Mode - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-range` - Range Tool - **Not Implemented**
*   `/ardour/MouseMode/set-mouse-mode-timefx` - Time FX Tool - **Not Implemented**
*   `/ardour/Notes/add-select-next` - Add Next to Selection - **Not Implemented**
*   `/ardour/Notes/add-select-previous` - Add Previous to Selection - **Not Implemented**
*   `/ardour/Notes/alt-add-select-next` - Add Next to Selection (alternate) - **Not Implemented**
*   `/ardour/Notes/alt-add-select-previous` - Add Previous to Selection (alternate) - **Not Implemented**
*   `/ardour/Notes/alt-delete` - Delete Selection (alternate) - **Not Implemented**
*   `/ardour/Notes/alt-select-next` - Select Next (alternate) - **Not Implemented**
*   `/ardour/Notes/alt-select-previous` - Select Previous (alternate) - **Not Implemented**
*   `/ardour/Notes/clear-selection` - Clear Note Selection - **Not Implemented**
*   `/ardour/Notes/decrease-velocity` - Decrease Velocity - **Not Implemented**
*   `/ardour/Notes/decrease-velocity-fine` - Decrease Velocity (fine) - **Not Implemented**
*   `/ardour/Notes/decrease-velocity-fine-smush` - Decrease Velocity (fine, allow mush) - **Not Implemented**
*   `/ardour/Notes/decrease-velocity-fine-smush-together` - Decrease Velocity (fine, allow mush, non-relative) - **Not Implemented**
*   `/ardour/Notes/decrease-velocity-fine-together` - Decrease Velocity (fine, non-relative) - **Not Implemented**
*   `/ardour/Notes/decrease-velocity-smush` - Decrease Velocity (allow mush) - **Not Implemented**
*   `/ardour/Notes/decrease-velocity-smush-together` - Decrease Velocity (maintain ratios, allow mush) - **Not Implemented**
*   `/ardour/Notes/decrease-velocity-together` - Decrease Velocity (non-relative) - **Not Implemented**
*   `/ardour/Notes/delete` - Delete Selection - **Not Implemented**
*   `/ardour/Notes/duplicate-selection` - Duplicate Note Selection - **Not Implemented**
*   `/ardour/Notes/edit-channels` - Edit Note Channels - **Not Implemented**
*   `/ardour/Notes/edit-velocities` - Edit Note Velocities - **Not Implemented**
*   `/ardour/Notes/extend-selection` - Extend Note Selection - **Not Implemented**
*   `/ardour/Notes/increase-velocity` - Increase Velocity - **Not Implemented**
*   `/ardour/Notes/increase-velocity-fine` - Increase Velocity (fine) - **Not Implemented**
*   `/ardour/Notes/increase-velocity-fine-smush` - Increase Velocity (fine, allow mush) - **Not Implemented**
*   `/ardour/Notes/increase-velocity-fine-smush-together` - Increase Velocity (fine, allow mush, non-relative) - **Not Implemented**
*   `/ardour/Notes/increase-velocity-fine-together` - Increase Velocity (fine, non-relative) - **Not Implemented**
*   `/ardour/Notes/increase-velocity-smush` - Increase Velocity (allow mush) - **Not Implemented**
*   `/ardour/Notes/increase-velocity-smush-together` - Increase Velocity (maintain ratios, allow mush) - **Not Implemented**
*   `/ardour/Notes/increase-velocity-together` - Increase Velocity (non-relative) - **Not Implemented**
*   `/ardour/Notes/invert-selection` - Invert Note Selection - **Not Implemented**
*   `/ardour/Notes/move-ends-earlier` - Move Note Ends Earlier - **Not Implemented**
*   `/ardour/Notes/move-ends-earlier-fine` - Move Note Ends Earlier (fine) - **Not Implemented**
*   `/ardour/Notes/move-ends-later` - Move Note Ends Later - **Not Implemented**
*   `/ardour/Notes/move-ends-later-fine` - Move Note Ends Later (fine) - **Not Implemented**
*   `/ardour/Notes/move-starts-earlier` - Move Note Start Earlier - **Not Implemented**
*   `/ardour/Notes/move-starts-earlier-fine` - Move Note Start Earlier (fine) - **Not Implemented**
*   `/ardour/Notes/move-starts-later` - Move Note Start Later - **Not Implemented**
*   `/ardour/Notes/move-starts-later-fine` - Move Note Start Later (fine) - **Not Implemented**
*   `/ardour/Notes/nudge-earlier` - Nudge Notes Earlier (grid) - **Not Implemented**
*   `/ardour/Notes/nudge-earlier-fine` - Nudge Notes Earlier (1/4 grid) - **Not Implemented**
*   `/ardour/Notes/nudge-later` - Nudge Notes Later (grid) - **Not Implemented**
*   `/ardour/Notes/nudge-later-fine` - Nudge Notes Later (1/4 grid) - **Not Implemented**
*   `/ardour/Notes/quantize-selected-notes` - Quantize Selected Notes - **Not Implemented**
*   `/ardour/Notes/select-next` - Select Next - **Not Implemented**
*   `/ardour/Notes/select-previous` - Select Previous - **Not Implemented**
*   `/ardour/Notes/transpose-down-octave` - Transpose Down (octave) - **Not Implemented**
*   `/ardour/Notes/transpose-down-octave-smush` - Transpose Down (octave, allow mush) - **Not Implemented**
*   `/ardour/Notes/transpose-down-semitone` - Transpose Down (semitone) - **Not Implemented**
*   `/ardour/Notes/transpose-down-semitone-smush` - Transpose Down (semitone, allow mush) - **Not Implemented**
*   `/ardour/Notes/transpose-up-octave` - Transpose Up (octave) - **Not Implemented**
*   `/ardour/Notes/transpose-up-octave-smush` - Transpose Up (octave, allow mush) - **Not Implemented**
*   `/ardour/Notes/transpose-up-semitone` - Transpose Up (semitone) - **Not Implemented**
*   `/ardour/Notes/transpose-up-semitone-smush` - Transpose Up (semitone, allow mush) - **Not Implemented**
*   `/ardour/Options/SendMMC` - Send MMC - **Not Implemented**
*   `/ardour/Options/SendMTC` - Send MTC - **Not Implemented**
*   `/ardour/Options/SendMidiClock` - Send MIDI Clock - **Not Implemented**
*   `/ardour/Options/UseMMC` - Use MMC - **Not Implemented**
*   `/ardour/ProcessorMenu/ab_plugins` - A/B Plugins - **Not Implemented**
*   `/ardour/ProcessorMenu/activate_all` - Activate All - **Not Implemented**
*   `/ardour/ProcessorMenu/backspace` - Delete - **Not Implemented**
*   `/ardour/ProcessorMenu/clear` - Clear (all) - **Not Implemented**
*   `/ardour/ProcessorMenu/clear_post` - Clear (post-fader) - **Not Implemented**
*   `/ardour/ProcessorMenu/clear_pre` - Clear (pre-fader) - **Not Implemented**
*   `/ardour/ProcessorMenu/controls` - Controls - **Not Implemented**
*   `/ardour/ProcessorMenu/copy` - Copy - **Not Implemented**
*   `/ardour/ProcessorMenu/custom-volume-pos` - Custom LAN Amp Position - **Not Implemented**
*   `/ardour/ProcessorMenu/cut` - Cut - **Not Implemented**
*   `/ardour/ProcessorMenu/deactivate_all` - Deactivate All - **Not Implemented**
*   `/ardour/ProcessorMenu/delete` - Delete - **Not Implemented**
*   `/ardour/ProcessorMenu/deselectall` - Deselect All - **Not Implemented**
*   `/ardour/ProcessorMenu/disk-io-custom` - Custom - **Not Implemented**
*   `/ardour/ProcessorMenu/disk-io-menu` - Disk I/O ... - **Not Implemented**
*   `/ardour/ProcessorMenu/disk-io-postfader` - Post-Fader - **Not Implemented**
*   `/ardour/ProcessorMenu/disk-io-prefader` - Pre-Fader - **Not Implemented**
*   `/ardour/ProcessorMenu/edit` - Edit... - **Not Implemented**
*   `/ardour/ProcessorMenu/edit-generic` - Edit with generic controls... - **Not Implemented**
*   `/ardour/ProcessorMenu/manage-pins` - Pin Connections... - **Not Implemented**
*   `/ardour/ProcessorMenu/newaux` - New Aux Send ... - **Not Implemented**
*   `/ardour/ProcessorMenu/newinsert` - New Insert - **Not Implemented**
*   `/ardour/ProcessorMenu/newlisten` - New Foldback Send ... - **Not Implemented**
*   `/ardour/ProcessorMenu/newplugin` - New Plugin - **Not Implemented**
*   `/ardour/ProcessorMenu/newsend` - New External Send ... - **Not Implemented**
*   `/ardour/ProcessorMenu/paste` - Paste - **Not Implemented**
*   `/ardour/ProcessorMenu/presets` - Presets - **Not Implemented**
*   `/ardour/ProcessorMenu/removelisten` - Remove Foldback Send ... - **Not Implemented**
*   `/ardour/ProcessorMenu/rename` - Rename - **Not Implemented**
*   `/ardour/ProcessorMenu/selectall` - Select All - **Not Implemented**
*   `/ardour/ProcessorMenu/send_options` - Send Options - **Not Implemented**
*   `/ardour/Recorder/arm-all` - Record Arm All Tracks - **Not Implemented**
*   `/ardour/Recorder/arm-none` - Disable Record Arm of All Tracks - **Not Implemented**
*   `/ardour/Recorder/reset-input-peak-hold` - Reset Input Peak Hold - **Not Implemented**
*   `/ardour/Region/add-range-marker-from-region` - Add Single Range Marker - **Not Implemented**
*   `/ardour/Region/add-range-markers-from-region` - Add Range Marker Per Region - **Not Implemented**
*   `/ardour/Region/add-region-cue-marker` - Add Region Cue Marker - **Not Implemented**
*   `/ardour/Region/align-regions-end` - Align End - **Not Implemented**
*   `/ardour/Region/align-regions-end-relative` - Align End Relative - **Not Implemented**
*   `/ardour/Region/align-regions-start` - Align Start - **Not Implemented**
*   `/ardour/Region/align-regions-start-relative` - Align Start Relative - **Not Implemented**
*   `/ardour/Region/align-regions-sync` - Align Sync - **Not Implemented**
*   `/ardour/Region/align-regions-sync-relative` - Align Sync Relative - **Not Implemented**
*   `/ardour/Region/alternate-nudge-backward` - Nudge Earlier - **Not Implemented**
*   `/ardour/Region/alternate-nudge-forward` - Nudge Later - **Not Implemented**
*   `/ardour/Region/alternate-set-fade-in-length` - Set Fade In Length - **Not Implemented**
*   `/ardour/Region/alternate-set-fade-out-length` - Set Fade Out Length - **Not Implemented**
*   `/ardour/Region/boost-region-gain` - Boost Gain - **Not Implemented**
*   `/ardour/Region/bounce-regions-processed` - Bounce (with processing) - **Not Implemented**
*   `/ardour/Region/bounce-regions-unprocessed` - Bounce (without processing) - **Not Implemented**
*   `/ardour/Region/choose-top-region` - Choose Top... - **Not Implemented**
*   `/ardour/Region/choose-top-region-context-menu` - Choose Top... - **Not Implemented**
*   `/ardour/Region/clear-region-cue-markers` - Clear Region Cue Markers - **Not Implemented**
*   `/ardour/Region/close-region-gaps` - Close Gaps - **Not Implemented**
*   `/ardour/Region/combine-regions` - Combine - **Not Implemented**
*   `/ardour/Region/cut-region-gain` - Cut Gain - **Not Implemented**
*   `/ardour/Region/deinterlace-midi` - Deinterlace Into Layers - **Not Implemented**
*   `/ardour/Region/duplicate-region` - Duplicate - **Not Implemented**
*   `/ardour/Region/export-region` - Export... - **Not Implemented**
*   `/ardour/Region/fork-region` - Unlink all selected regions - **Not Implemented**
*   `/ardour/Region/fork-regions-from-unselected` - Unlink from unselected - **Not Implemented**
*   `/ardour/Region/insert-patch-change` - Insert Patch Change... - **Not Implemented**
*   `/ardour/Region/insert-patch-change-context` - Insert Patch Change... - **Not Implemented**
*   `/ardour/Region/insert-region-from-source-list` - Insert Region from Source List - **Not Implemented**
*   `/ardour/Region/legatize-region` - Legatize - **Not Implemented**
*   `/ardour/Region/loop-region` - Loop - **Not Implemented**
*   `/ardour/Region/loudness-analyze-region` - Loudness Analysis... - **Not Implemented**
*   `/ardour/Region/lower-region` - Lower - **Not Implemented**
*   `/ardour/Region/lower-region-to-bottom` - Lower to Bottom - **Not Implemented**
*   `/ardour/Region/make-region-markers-cd` - Convert Region Cue Markers to CD Markers - **Not Implemented**
*   `/ardour/Region/make-region-markers-global` - Convert Region Cue Markers to Location Markers - **Not Implemented**
*   `/ardour/Region/multi-duplicate-region` - Multi-Duplicate... - **Not Implemented**
*   `/ardour/Region/naturalize-region` - Move to Original Position - **Not Implemented**
*   `/ardour/Region/normalize-region` - Normalize... - **Not Implemented**
*   `/ardour/Region/nudge-backward` - Nudge Earlier - **Not Implemented**
*   `/ardour/Region/nudge-backward-by-capture-offset` - Nudge Earlier by Capture Offset - **Not Implemented**
*   `/ardour/Region/nudge-forward` - Nudge Later - **Not Implemented**
*   `/ardour/Region/nudge-forward-by-capture-offset` - Nudge Later by Capture Offset - **Not Implemented**
*   `/ardour/Region/pitch-shift-region` - Pitch Shift... - **Not Implemented**
*   `/ardour/Region/place-transient` - Place Transient - **Not Implemented**
*   `/ardour/Region/play-selected-regions` - Play Selected Regions - **Not Implemented**
*   `/ardour/Region/quantize-region` - Quantize... - **Not Implemented**
*   `/ardour/Region/raise-region` - Raise - **Not Implemented**
*   `/ardour/Region/raise-region-to-top` - Raise to Top - **Not Implemented**
*   `/ardour/Region/region-fill-track` - Fill Track - **Not Implemented**
*   `/ardour/Region/remove-overlap` - Remove Overlap - **Not Implemented**
*   `/ardour/Region/remove-region` - Remove - **Not Implemented**
*   `/ardour/Region/remove-region-sync` - Remove Sync - **Not Implemented**
*   `/ardour/Region/rename-region` - Rename... - **Not Implemented**
*   `/ardour/Region/reset-region-gain` - Reset Gain - **Not Implemented**
*   `/ardour/Region/reset-region-gain-envelopes` - Reset Envelope - **Not Implemented**
*   `/ardour/Region/reverse-region` - Reverse - **Not Implemented**
*   `/ardour/Region/separate-under-region` - Separate Under - **Not Implemented**
*   `/ardour/Region/sequence-regions` - Sequence Regions - **Not Implemented**
*   `/ardour/Region/set-fade-in-length` - Set Fade In Length - **Not Implemented**
*   `/ardour/Region/set-fade-out-length` - Set Fade Out Length - **Not Implemented**
*   `/ardour/Region/set-loop-from-region` - Set Loop Range - **Not Implemented**
*   `/ardour/Region/set-punch-from-region` - Set Punch - **Not Implemented**
*   `/ardour/Region/set-region-sync-position` - Set Sync Position - **Not Implemented**
*   `/ardour/Region/set-selection-from-region` - Set Range Selection - **Not Implemented**
*   `/ardour/Region/set-tempo-from-region` - Set Tempo from Region = Bar - **Not Implemented**
*   `/ardour/Region/show-region-list-editor` - List Editor... - **Not Implemented**
*   `/ardour/Region/show-region-properties` - Properties... - **Not Implemented**
*   `/ardour/Region/show-rhythm-ferret` - Rhythm Ferret... - **Not Implemented**
*   `/ardour/Region/snap-regions-to-grid` - Snap Position to Grid - **Not Implemented**
*   `/ardour/Region/spectral-analyze-region` - Spectral Analysis... - **Not Implemented**
*   `/ardour/Region/split-multichannel-region` - Make Mono Regions - **Not Implemented**
*   `/ardour/Region/split-region-at-transients` - Split at Percussion Onsets - **Not Implemented**
*   `/ardour/Region/strip-region-silence` - Strip Silence... - **Not Implemented**
*   `/ardour/Region/tag-selected-regions` - Tag Selected Regions - **Not Implemented**
*   `/ardour/Region/toggle-opaque-region` - Opaque - **Not Implemented**
*   `/ardour/Region/toggle-region-fade-in` - Fade In - **Not Implemented**
*   `/ardour/Region/toggle-region-fade-out` - Fade Out - **Not Implemented**
*   `/ardour/Region/toggle-region-fades` - Fades - **Not Implemented**
*   `/ardour/Region/toggle-region-polarity` - Invert Polarity - **Not Implemented**
*   `/ardour/Region/toggle-region-gain-envelope-active` - Envelope Active - **Not Implemented**
*   `/ardour/Region/toggle-region-lock` - Lock - **Not Implemented**
*   `/ardour/Region/toggle-region-mute` - Mute - **Not Implemented**
*   `/ardour/Region/toggle-region-video-lock` - Lock to Video - **Not Implemented**
*   `/ardour/Region/transform-region` - Transform... - **Not Implemented**
*   `/ardour/Region/transpose-region` - Transpose... - **Not Implemented**
*   `/ardour/Region/trim-back` - Trim End at Edit Point - **Not Implemented**
*   `/ardour/Region/trim-front` - Trim Start at Edit Point - **Not Implemented**
*   `/ardour/Region/trim-region-to-loop` - Trim to Loop - **Not Implemented**
*   `/ardour/Region/trim-region-to-punch` - Trim to Punch - **Not Implemented**
*   `/ardour/Region/trim-to-next-region` - Trim to Next - **Not Implemented**
*   `/ardour/Region/trim-to-previous-region` - Trim to Previous - **Not Implemented**
*   `/ardour/Region/uncombine-regions` - Uncombine - **Not Implemented**
*   `/ardour/RegionList/removeUnusedRegions` - Remove Unused - **Not Implemented**
*   `/ardour/RegionList/rlAudition` - Audition - **Not Implemented**
*   `/ardour/Rulers/toggle-bbt-ruler` - Bars:Beats - **Not Implemented**
*   `/ardour/Rulers/toggle-cd-marker-ruler` - CD Markers - **Not Implemented**
*   `/ardour/Rulers/toggle-cue-marker-ruler` - Cue Markers - **Not Implemented**
*   `/ardour/Rulers/toggle-loop-punch-ruler` - Loop/Punch Ranges - **Not Implemented**
*   `/ardour/Rulers/toggle-marker-ruler` - Location Markers - **Not Implemented**
*   `/ardour/Rulers/toggle-meter-ruler` - Time Signature - **Not Implemented**
*   `/ardour/Rulers/toggle-minsec-ruler` - Mins:Secs - **Not Implemented**
*   `/ardour/Rulers/toggle-range-ruler` - Range Markers - **Not Implemented**
*   `/ardour/Rulers/toggle-samples-ruler` - Samples - **Not Implemented**
*   `/ardour/Rulers/toggle-tempo-ruler` - Tempo - **Not Implemented**
*   `/ardour/Rulers/toggle-timecode-ruler` - Timecode - **Not Implemented**
*   `/ardour/Rulers/toggle-video-ruler` - Video Timeline - **Not Implemented**
*   `/ardour/Snap/grid-type-asixteenthbeat` - 1/64 Note - **Not Implemented**
*   `/ardour/Snap/grid-type-bar` - Bar - **Not Implemented**
*   `/ardour/Snap/grid-type-beat` - 1/4 Note - **Not Implemented**
*   `/ardour/Snap/grid-type-cdframe` - CD Frames - **Not Implemented**
*   `/ardour/Snap/grid-type-eighths` - 1/32 Note - **Not Implemented**
*   `/ardour/Snap/grid-type-fifths` - 1/5 (8th quintuplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-fourteenths` - 1/14 (16th septuplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-halves` - 1/8 Note - **Not Implemented**
*   `/ardour/Snap/grid-type-minsec` - MinSec - **Not Implemented**
*   `/ardour/Snap/grid-type-none` - No Grid - **Not Implemented**
*   `/ardour/Snap/grid-type-quarters` - 1/16 Note - **Not Implemented**
*   `/ardour/Snap/grid-type-sevenths` - 1/7 (8th septuplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-sixths` - 1/6 (16th triplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-tenths` - 1/10 (16th quintuplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-thirds` - 1/3 (8th triplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-thirtyseconds` - 1/128 Note - **Not Implemented**
*   `/ardour/Snap/grid-type-timecode` - Timecode - **Not Implemented**
*   `/ardour/Snap/grid-type-twelfths` - 1/12 (32nd triplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-twentieths` - 1/20 (32nd quintuplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-twentyeighths` - 1/28 (32nd septuplet) - **Not Implemented**
*   `/ardour/Snap/grid-type-twentyfourths` - 1/24 (64th triplet) - **Not Implemented**
*   `/ardour/Solo/solo-use-afl` - After Fade Listen (AFL) solo - **Not Implemented**
*   `/ardour/Solo/solo-use-in-place` - In-place solo - **Not Implemented**
*   `/ardour/Solo/solo-use-pfl` - Pre Fade Listen (PFL) solo - **Not Implemented**
*   `/ardour/Solo/toggle-exclusive-solo` - Toggle exclusive solo mode - **Not Implemented**
*   `/ardour/Solo/toggle-mute-overrides-solo` - Toggle mute overrides solo mode - **Not Implemented**
*   `/ardour/StepEditing/back` - Move Insert Position Back by Note Length - **Not Implemented**
*   `/ardour/StepEditing/dec-note-length` - Decrease Note Length - **Not Implemented**
*   `/ardour/StepEditing/dec-note-velocity` - Decrease Note Velocity - **Not Implemented**
*   `/ardour/StepEditing/inc-note-length` - Increase Note Length - **Not Implemented**
*   `/ardour/StepEditing/inc-note-velocity` - Increase Note Velocity - **Not Implemented**
*   `/ardour/StepEditing/insert-a` - Insert Note A - **Not Implemented**
*   `/ardour/StepEditing/insert-asharp` - Insert Note A-sharp - **Not Implemented**
*   `/ardour/StepEditing/insert-b` - Insert Note B - **Not Implemented**
*   `/ardour/StepEditing/insert-c` - Insert Note C - **Not Implemented**
*   `/ardour/StepEditing/insert-csharp` - Insert Note C-sharp - **Not Implemented**
*   `/ardour/StepEditing/insert-d` - Insert Note D - **Not Implemented**
*   `/ardour/StepEditing/insert-dsharp` - Insert Note D-sharp - **Not Implemented**
*   `/ardour/StepEditing/insert-e` - Insert Note E - **Not Implemented**
*   `/ardour/StepEditing/insert-f` - Insert Note F - **Not Implemented**
*   `/ardour/StepEditing/insert-fsharp` - Insert Note F-sharp - **Not Implemented**
*   `/ardour/StepEditing/insert-g` - Insert Note G - **Not Implemented**
*   `/ardour/StepEditing/insert-gsharp` - Insert Note G-sharp - **Not Implemented**
*   `/ardour/StepEditing/insert-rest` - Insert a Note-length Rest - **Not Implemented**
*   `/ardour/StepEditing/insert-snap-rest` - Insert a Snap-length Rest - **Not Implemented**
*   `/ardour/StepEditing/next-note-length` - Move to Next Note Length - **Not Implemented**
*   `/ardour/StepEditing/next-note-velocity` - Move to Next Note Velocity - **Not Implemented**
*   `/ardour/StepEditing/next-octave` - Move to next octave - **Not Implemented**
*   `/ardour/StepEditing/no-dotted` - No Dotted Notes - **Not Implemented**
*   `/ardour/StepEditing/note-length-eighth` - Set Note Length to 1/8 - **Not Implemented**
*   `/ardour/StepEditing/note-length-half` - Set Note Length to 1/2 - **Not Implemented**
*   `/ardour/StepEditing/note-length-quarter` - Set Note Length to 1/4 - **Not Implemented**
*   `/ardour/StepEditing/note-length-sixteenth` - Set Note Length to 1/16 - **Not Implemented**
*   `/ardour/StepEditing/note-length-sixtyfourth` - Set Note Length to 1/64 - **Not Implemented**
*   `/ardour/StepEditing/note-length-third` - Set Note Length to 1/3 - **Not Implemented**
*   `/ardour/StepEditing/note-length-thirtysecond` - Set Note Length to 1/32 - **Not Implemented**
*   `/ardour/StepEditing/note-length-whole` - Set Note Length to Whole - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-f` - Set Note Velocity to Forte - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-ff` - Set Note Velocity to Fortississimo - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-fff` - Set Note Velocity to Fortississimo - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-mf` - Set Note Velocity to Mezzo-Forte - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-mp` - Set Note Velocity to Mezzo-Piano - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-p` - Set Note Velocity to Piano - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-pp` - Set Note Velocity to Pianissimo - **Not Implemented**
*   `/ardour/StepEditing/note-velocity-ppp` - Set Note Velocity to Pianississimo - **Not Implemented**
*   `/ardour/StepEditing/octave-0` - Switch to the 1st octave - **Not Implemented**
*   `/ardour/StepEditing/octave-1` - Switch to the 2nd octave - **Not Implemented**
*   `/ardour/StepEditing/octave-10` - Switch to the 11th octave - **Not Implemented**
*   `/ardour/StepEditing/octave-2` - Switch to the 3rd octave - **Not Implemented**
*   `/ardour/StepEditing/octave-3` - Switch to the 4th octave - **Not Implemented**
*   `/ardour/StepEditing/octave-4` - Switch to the 5th octave - **Not Implemented**
*   `/ardour/StepEditing/octave-5` - Switch to the 6th octave - **Not Implemented**
*   `/ardour/StepEditing/octave-6` - Switch to the 7th octave - **Not Implemented**
*   `/ardour/StepEditing/octave-7` - Switch to the 8th octave - **Not Implemented**
*   `/ardour/StepEditing/octave-8` - Switch to the 9th octave - **Not Implemented**
*   `/ardour/StepEditing/octave-9` - Switch to the 10th octave - **Not Implemented**
*   `/ardour/StepEditing/prev-note-length` - Move to Previous Note Length - **Not Implemented**
*   `/ardour/StepEditing/prev-note-velocity` - Move to Previous Note Velocity - **Not Implemented**
*   `/ardour/StepEditing/prev-octave` - Move to next octave - **Not Implemented**
*   `/ardour/StepEditing/sustain` - Sustain Selected Notes by Note Length - **Not Implemented**
*   `/ardour/StepEditing/sync-to-edit-point` - Move Insert Position to Edit Point - **Not Implemented**
*   `/ardour/StepEditing/toggle-chord` - Toggle Chord Entry - **Not Implemented**
*   `/ardour/StepEditing/toggle-dotted` - Toggled Dotted Notes - **Not Implemented**
*   `/ardour/StepEditing/toggle-double-dotted` - Toggled Double-Dotted Notes - **Not Implemented**
*   `/ardour/StepEditing/toggle-triple-dotted` - Toggled Triple-Dotted Notes - **Not Implemented**
*   `/ardour/StepEditing/toggle-triplet` - Toggle Triple Notes - **Not Implemented**
*   `/ardour/Transport/Forward` - Forward - **Not Implemented**
*   `/ardour/Transport/ForwardFast` - Forward (Fast) - **Not Implemented**
*   `/ardour/Transport/ForwardSlow` - Forward (Slow) - **Not Implemented**
*   `/ardour/Transport/GotoEnd` - Go to End - **Not Implemented**
*   `/ardour/Transport/GotoStart` - Go to Start - **Not Implemented**
*   `/ardour/Transport/GotoWallClock` - Go to Wall Clock - **Not Implemented**
*   `/ardour/Transport/GotoZero` - Go to Zero - **Not Implemented**
*   `/ardour/Transport/Loop` - Play Loop Range - **Not Implemented**
*   `/ardour/Transport/PlayPreroll` - Play w/Preroll - **Not Implemented**
*   `/ardour/Transport/PlaySelection` - Play Selection - **Not Implemented**
*   `/ardour/Transport/Record` - Enable Record - **Not Implemented**
*   `/ardour/Transport/RecordCountIn` - Record w/Count-In - **Not Implemented**
*   `/ardour/Transport/RecordPreroll` - Record w/Preroll - **Not Implemented**
*   `/ardour/Transport/Rewind` - Rewind - **Not Implemented**
*   `/ardour/Transport/RewindFast` - Rewind (Fast) - **Not Implemented**
*   `/ardour/Transport/RewindSlow` - Rewind (Slow) - **Not Implemented**
*   `/ardour/Transport/Roll` - Roll - **Not Implemented**
*   `/ardour/Transport/SessionMonitorDisk` - All Disk - **Not Implemented**
*   `/ardour/Transport/SessionMonitorIn` - All Input - **Not Implemented**
*   `/ardour/Transport/Stop` - Stop - **Not Implemented**
*   `/ardour/Transport/ToggleAutoInput` - Auto Input - **Not Implemented**
*   `/ardour/Transport/ToggleAutoPlay` - Auto Play - **Not Implemented**
*   `/ardour/Transport/ToggleAutoReturn` - Auto Return - **Not Implemented**
*   `/ardour/Transport/ToggleClick` - Click - **Not Implemented**
*   `/ardour/Transport/ToggleExternalSync` - Use External Positional Sync Source - **Not Implemented**
*   `/ardour/Transport/ToggleFollowEdits` - Follow Range - **Not Implemented**
*   `/ardour/Transport/TogglePunch` - Punch In/Out - **Not Implemented**
*   `/ardour/Transport/TogglePunchIn` - Punch In - **Not Implemented**
*   `/ardour/Transport/TogglePunchOut` - Punch Out - **Not Implemented**
*   `/ardour/Transport/ToggleRoll` - Start/Stop - **Not Implemented**
*   `/ardour/Transport/ToggleRollForgetCapture` - Stop and Forget Capture - **Not Implemented**
*   `/ardour/Transport/ToggleRollMaybe` - Start/Continue/Stop - **Not Implemented**
*   `/ardour/Transport/ToggleTimeMaster` - Time Master - **Not Implemented**
*   `/ardour/Transport/ToggleVideoSync` - Sync Startup to Video - **Not Implemented**
*   `/ardour/Transport/TransitionToReverse` - Transition to Reverse - **Not Implemented**
*   `/ardour/Transport/TransitionToRoll` - Transition to Roll - **Not Implemented**
*   `/ardour/Transport/Transport` - Transport - **Not Implemented**
*   `/ardour/Transport/alternate-GotoStart` - Go to Start - **Not Implemented**
*   `/ardour/Transport/alternate-ToggleRoll` - Start/Stop - **Not Implemented**
*   `/ardour/Transport/alternate-numpad-decimal` - Numpad Decimal - **Not Implemented**
*   `/ardour/Transport/alternate-record-roll` - Start Recording - **Not Implemented**
*   `/ardour/Transport/focus-on-clock` - Focus On Clock - **Not Implemented**
*   `/ardour/Transport/goto-mark-1` - Locate to Mark 1 - **Not Implemented**
*   `/ardour/Transport/goto-mark-2` - Locate to Mark 2 - **Not Implemented**
*   `/ardour/Transport/goto-mark-3` - Locate to Mark 3 - **Not Implemented**
*   `/ardour/Transport/goto-mark-4` - Locate to Mark 4 - **Not Implemented**
*   `/ardour/Transport/goto-mark-5` - Locate to Mark 5 - **Not Implemented**
*   `/ardour/Transport/goto-mark-6` - Locate to Mark 6 - **Not Implemented**
*   `/ardour/Transport/goto-mark-7` - Locate to Mark 7 - **Not Implemented**
*   `/ardour/Transport/goto-mark-8` - Locate to Mark 8 - **Not Implemented**
*   `/ardour/Transport/goto-mark-9` - Locate to Mark 9 - **Not Implemented**
*   `/ardour/Transport/numpad-0` - Numpad 0 - **Not Implemented**
*   `/ardour/Transport/numpad-1` - Numpad 1 - **Not Implemented**
*   `/ardour/Transport/numpad-2` - Numpad 2 - **Not Implemented**
*   `/ardour/Transport/numpad-3` - Numpad 3 - **Not Implemented**
*   `/ardour/Transport/numpad-4` - Numpad 4 - **Not Implemented**
*   `/ardour/Transport/numpad-5` - Numpad 5 - **Not Implemented**
*   `/ardour/Transport/numpad-6` - Numpad 6 - **Not Implemented**
*   `/ardour/Transport/numpad-7` - Numpad 7 - **Not Implemented**
*   `/ardour/Transport/numpad-8` - Numpad 8 - **Not Implemented**
*   `/ardour/Transport/numpad-9` - Numpad 9 - **Not Implemented**
*   `/ardour/Transport/numpad-decimal` - Numpad Decimal - **Not Implemented**
*   `/ardour/Transport/primary-clock-bbt` - Bars & Beats - **Not Implemented**
*   `/ardour/Transport/primary-clock-minsec` - Minutes & Seconds - **Not Implemented**
*   `/ardour/Transport/primary-clock-samples` - Samples - **Not Implemented**
*   `/ardour/Transport/primary-clock-seconds` - Seconds - **Not Implemented**
*   `/ardour/Transport/primary-clock-timecode` - Timecode - **Not Implemented**
*   `/ardour/Transport/record-roll` - Start Recording - **Not Implemented**
*   `/ardour/Transport/secondary-clock-bbt` - Bars & Beats - **Not Implemented**
*   `/ardour/Transport/secondary-clock-minsec` - Minutes & Seconds - **Not Implemented**
*   `/ardour/Transport/secondary-clock-samples` - Samples - **Not Implemented**
*   `/ardour/Transport/secondary-clock-seconds` - Seconds - **Not Implemented**
*   `/ardour/Transport/secondary-clock-timecode` - Timecode - **Not Implemented**
*   `/ardour/Transport/solo-selection` - Solo Selection - **Not Implemented**
*   `/ardour/Window/toggle-about` - About - **Not Implemented**
*   `/ardour/Window/toggle-add-routes` - Add Tracks/Busses - **Not Implemented**
*   `/ardour/Window/toggle-add-video` - Add Video - **Not Implemented**
*   `/ardour/Window/toggle-audio-connection-manager` - Audio Connections - **Not Implemented**
*   `/ardour/Window/toggle-audio-midi-setup` - Audio/MIDI Setup - **Not Implemented**
*   `/ardour/Window/toggle-big-clock` - Big Clock - **Not Implemented**
*   `/ardour/Window/toggle-big-transport` - Transport Controls - **Not Implemented**
*   `/ardour/Window/toggle-bundle-manager` - Bundle Manager - **Not Implemented**
*   `/ardour/Window/toggle-dsp-statistics` - Performance Meters - **Not Implemented**
*   `/ardour/Window/toggle-idle-o-meter` - Idle'o'Meter - **Not Implemented**
*   `/ardour/Window/toggle-inspector` - Tracks and Busses - **Not Implemented**
*   `/ardour/Window/toggle-io-plugins` - I/O Plugins - **Not Implemented**
*   `/ardour/Window/toggle-key-editor` - Keyboard Shortcuts - **Not Implemented**
*   `/ardour/Window/toggle-library-downloader` - Library Downloader - **Not Implemented**
*   `/ardour/Window/toggle-locations` - Locations - **Not Implemented**
*   `/ardour/Window/toggle-luawindow` - Scripting - **Not Implemented**
*   `/ardour/Window/toggle-midi-connection-manager` - MIDI Connections - **Not Implemented**
*   `/ardour/Window/toggle-plugin-dsp-load` - Plugin DSP Load - **Not Implemented**
*   `/ardour/Window/toggle-plugin-manager` - Plugin Manager - **Not Implemented**
*   `/ardour/Window/toggle-script-manager` - Script Manager - **Not Implemented**
*   `/ardour/Window/toggle-session-options-editor` - Properties - **Not Implemented**
*   `/ardour/Window/toggle-speaker-config` - Speaker Configuration - **Not Implemented**
*   `/ardour/Window/toggle-transport-masters` - Transport Masters - **Not Implemented**
*   `/ardour/Window/toggle-video-export` - Video Export Dialog - **Not Implemented**
*   `/ardour/Window/toggle-virtual-keyboard` - Virtual Keyboard - **Not Implemented**
*   `/ardour/Zoom/zoom-focus-center` - Zoom Focus Center - **Not Implemented**
*   `/ardour/Zoom/zoom-focus-edit` - Zoom Focus Edit Point - **Not Implemented**
*   `/ardour/Zoom/zoom-focus-left` - Zoom Focus Left - **Not Implemented**
*   `/ardour/Zoom/zoom-focus-mouse` - Zoom Focus Mouse - **Not Implemented**
*   `/ardour/Zoom/zoom-focus-playhead` - Zoom Focus Playhead - **Not Implemented**
*   `/ardour/Zoom/zoom-focus-right` - Zoom Focus Right - **Not Implemented**
