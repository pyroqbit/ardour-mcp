use schemars::JsonSchema; // Ensure necessary imports are present
use serde::{Deserialize, Serialize}; // Ensure necessary imports are present

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct TrackInfo {
    pub id: i32,
    pub name: String,
    pub track_type: String, // e.g., "audio", "midi", "bus"
} 