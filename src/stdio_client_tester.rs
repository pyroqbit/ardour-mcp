use anyhow::Result;
use rmcp::{model::{CallToolRequestParam, ListToolsResult, ReadResourceRequestParam, ReadResourceResult, CallToolResult}, service::{ServiceExt, RoleClient, RunningService, ServiceError}, transport::TokioChildProcess, Error as McpError};
use serde_json::{json, Value as JsonValue, Map};
use tokio::process::Command;
use std::{
    fs::OpenOptions,
    path::Path,
    io::Stderr,
    path::PathBuf,
};
use tracing_subscriber::{fmt::writer::{MakeWriterExt, Tee}, EnvFilter};
use chrono;

async fn read_playback_state_helper(service: &RunningService<RoleClient, ()>) -> Result<ReadResourceResult, ServiceError> {
    let resource_uri = "ardour:/state/playback";
    tracing::info!("Attempting to read resource: {}", resource_uri);
    let result = service
        .read_resource(ReadResourceRequestParam {
            uri: resource_uri.to_string(),
            // Здесь можно добавить другие параметры ReadResourceRequestParam, если они понадобятся
        })
        .await;
    match result {
        Ok(resource_result) => {
            tracing::info!("Resource '{}' read successfully. Result: {:#?}", resource_uri, resource_result);
            Ok(resource_result)
        }
        Err(service_error) => {
            tracing::error!("ServiceError reading resource '{}': {:?}", resource_uri, service_error);
            Err(service_error)
        }
    }
}

async fn read_resource_to_string_helper(service: &RunningService<RoleClient, ()>, resource_uri: &str) -> Result<String, String> {
    tracing::info!("Attempting to read resource: {}", resource_uri);
    let result = service
        .read_resource(ReadResourceRequestParam {
            uri: resource_uri.to_string(),
        })
        .await;
    match result {
        Ok(resource_result) => {
            if let Some(content_wrapper) = resource_result.contents.first() {
                match content_wrapper {
                    rmcp::model::ResourceContents::TextResourceContents { text, mime_type, .. } => {
                        let actual_mime_type = mime_type.as_deref().unwrap_or("text/plain"); // Default if None
                        tracing::info!("Resource '{}' read successfully. Mime: '{}', Text: {}", resource_uri, actual_mime_type, text);
                        // We expect either text/plain or application/json, both will be in the 'text' field.
                        Ok(text.clone())
                    }
                    rmcp::model::ResourceContents::BlobResourceContents { mime_type, .. } => {
                        // We are not expecting blob content for these resources in the tester
                        let actual_mime_type = mime_type.as_deref().unwrap_or("application/octet-stream");
                        Err(format!("Resource '{}' read, but content was unexpected Blob. Mime: {}", resource_uri, actual_mime_type))
                    }
                    // Add other variants here if ResourceContents has more than Text and Blob
                }
            } else {
                Err(format!("Resource '{}' read, but no content returned.", resource_uri))
            }
        }
        Err(service_error) => {
            let err_msg = format!("ServiceError reading resource '{}': {:?}", resource_uri, service_error);
            tracing::error!("{}", err_msg);
            Err(err_msg)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create log directory if it doesn't exist for the client
    let log_dir = Path::new("logs");
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir)?;
    }
    let client_log_file_path = log_dir.join("stdio_client_tester.log");

    // Create or append to the client log file
    let client_log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(client_log_file_path)?;

    // Create a combined writer for file (debug) and stderr (info)
    let stderr_make_writer = 
        (std::io::stderr as fn() -> Stderr).with_max_level(tracing::Level::INFO);
    let file_make_writer = 
        client_log_file.with_max_level(tracing::Level::DEBUG);
    let client_combined_writer = Tee::new(stderr_make_writer, file_make_writer);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into())) // Overall DEBUG level
        .with_writer(client_combined_writer) 
        .with_ansi(true) // Enable ANSI for stderr if terminal supports it
        .init();

    tracing::info!("\n======================================================================\nNEW CLIENT TESTER RUN: {}\n======================================================================", chrono::Local::now().to_rfc2822());
    tracing::info!("stdio_client_tester starting. Logging to logs/stdio_client_tester.log and terminal.");

    // Log current working directory
    match std::env::current_dir() {
        Ok(cwd) => tracing::info!("Current working directory: {}", cwd.display()),
        Err(e) => tracing::error!("Failed to get current working directory: {}", e),
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let server_exe_path = manifest_dir.join("target/debug/ardour_mcp_server");
    tracing::info!("Calculated server executable path: {}", server_exe_path.display());

    let mut server_command = Command::new(server_exe_path);

    let service = ()
        .serve(TokioChildProcess::new(&mut server_command)?)
        .await?;

    // Initialize
    let server_info = service.peer_info();
    tracing::info!("Connected to server: {server_info:#?}");

    // Read initial playback state
    if let Err(e) = read_playback_state_helper(&service).await {
        tracing::error!("Initial Read Playback State failed with ServiceError: {:?}", e);
    }

    // List tools
    let tools: ListToolsResult = service.list_tools(Default::default()).await?;
    tracing::info!("Available tools: {tools:#?}");

    // List all available resources
    tracing::info!("Listing all available resources...");
    match service.list_resources(None).await {
        Ok(list_result) => {
            tracing::info!("ListResources successful. Full result: {:?}", list_result);
            for resource in list_result.resources {
                tracing::info!(
                    "Resource: {:?}",
                    resource
                );
            }
        }
        Err(e) => {
            tracing::error!("Failed to list resources: {:?}", e);
        }
    }

    // Test reading ardour:/state/transport_frame resource
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Short delay
    tracing::info!("Attempting to read resource directly: ardour:/state/transport_frame");
    match read_resource_to_string_helper(&service, "ardour:/state/transport_frame").await {
        Ok(frame_string) => {
            tracing::info!("Read 'ardour:/state/transport_frame' successfully. Frame: {}", frame_string);

            // Test calling set_track_mute with the new path
            tracing::info!("Attempting to call set_track_mute for rid 1 to mute (true). Path should be /strip/mute.");
            let mute_args = json!({
                "rid": 1,
                "mute_state": true
            });
            match call_tool_with_json_args(&service, "set_track_mute", Some(mute_args)).await {
                Ok(result) => {
                    tracing::info!("Call to 'set_track_mute' successful. Result: {:?}", result);
                }
                Err(e) => {
                    tracing::error!("Failed to call 'set_track_mute': {}", e);
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to read 'ardour:/state/transport_frame': {:?}", e);
        }
    }

    // Test select_strip
    tracing::info!("Attempting to call select_strip for rid 2 to select (true).");
    let select_strip_args = json!({ "rid": 2, "select_state": true });
    match call_tool_with_json_args(&service, "select_strip", Some(select_strip_args)).await {
        Ok(result) => tracing::info!(
            "Call to 'select_strip' successful. Result: {:?}",
            result
        ),
        Err(e) => tracing::error!("Error calling 'select_strip': {:?}", e),
    }

    // Test set_strip_plugin_active
    tracing::info!("Attempting to call set_strip_plugin_active for rid 1, slot 1 to active (true).");
    let plugin_active_args = json!({ "rid": 1, "plugin_slot": 1, "active_state": true });
    match call_tool_with_json_args(&service, "set_strip_plugin_active", Some(plugin_active_args)).await {
        Ok(result) => tracing::info!(
            "Call to 'set_strip_plugin_active' successful. Result: {:?}",
            result
        ),
        Err(e) => tracing::error!("Error calling 'set_strip_plugin_active': {:?}", e),
    }

    // Test set_strip_plugin_parameter
    tracing::info!("Attempting to call set_strip_plugin_parameter for rid 1, slot 1, param 1 to 0.5.");
    let plugin_param_args = json!({ "rid": 1, "plugin_slot": 1, "param_id": 1, "value": 0.5f32 });
    match call_tool_with_json_args(&service, "set_strip_plugin_parameter", Some(plugin_param_args)).await {
        Ok(result) => tracing::info!(
            "Call to 'set_strip_plugin_parameter' successful. Result: {:?}",
            result
        ),
        Err(e) => tracing::error!("Error calling 'set_strip_plugin_parameter': {:?}", e),
    }

    // Test set_strip_pan_stereo_width
    tracing::info!("Attempting to call set_strip_pan_stereo_width for rid 1, width 0.75.");
    let pan_width_args = json!({ "rid": 1, "width": 0.75f32 });
    match call_tool_with_json_args(&service, "set_strip_pan_stereo_width", Some(pan_width_args)).await {
        Ok(result) => tracing::info!(
            "Call to 'set_strip_pan_stereo_width' successful. Result: {:?}",
            result
        ),
        Err(e) => tracing::error!("Error calling 'set_strip_pan_stereo_width': {:?}", e),
    }

    // Test set_selected_strip_pan_stereo_width
    // First, ensure a strip is selected (e.g., RID 1)
    tracing::info!("Attempting to select_strip for rid 1 (again, to be sure).");
    let select_args_for_width_test = json!({ "rid": 1, "select_state": true });
    match call_tool_with_json_args(&service, "select_strip", Some(select_args_for_width_test)).await {
        Ok(result) => tracing::info!(
            "Call to 'select_strip' (before selected_width test) successful. Result: {:?}",
            result
        ),
        Err(e) => tracing::error!("Error calling 'select_strip' (before selected_width test): {:?}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await; // Small delay if needed

    tracing::info!("Attempting to call set_selected_strip_pan_stereo_width for width 0.65.");
    let selected_pan_width_args = json!({ "width": 0.65f32 });
    match call_tool_with_json_args(&service, "set_selected_strip_pan_stereo_width", Some(selected_pan_width_args)).await {
        Ok(result) => tracing::info!(
            "Call to 'set_selected_strip_pan_stereo_width' successful. Result: {:?}",
            result
        ),
        Err(e) => tracing::error!("Error calling 'set_selected_strip_pan_stereo_width': {:?}", e),
    }

    // Keep a small delay at the end before shutdown to ensure messages are processed
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    tracing::info!("Simplified client operations complete. Shutting down.");

    Ok(())
}

/// Helper function to call a tool with JSON arguments
async fn call_tool_with_json_args(service: &RunningService<RoleClient, ()>, tool_name: &str, args: Option<serde_json::Value>) -> Result<CallToolResult, ServiceError> {
    let arguments_map: Option<Map<String, JsonValue>> = match args {
        Some(serde_json::Value::Object(map)) => Some(map),
        Some(_) => {
            // If args is Some but not an Object, this is an error for CallToolRequestParam
            // Alternatively, could return an error here or log and pass None
            tracing::error!("Tool arguments must be a JSON object if provided.");
            return Err(ServiceError::McpError(McpError::invalid_params("Tool arguments must be a JSON object".to_string(), None)));
        }
        None => None,
    };

    let params = CallToolRequestParam {
        name: tool_name.to_string().into(), // Corrected: .into() for Cow
        arguments: arguments_map,          // Corrected: Use Option<Map<String, JsonValue>>
        // annotations field removed
    };
    service.call_tool(params).await
} 