//! WebSocket protocol types for Rejoice Studio.
//!
//! Defines the message format for communication between the Studio UI
//! and the dev server.

use serde::{Deserialize, Serialize};

/// A single edit operation within a file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edit {
    /// Line number (1-indexed)
    pub line: u32,
    /// The text to find and replace on this line
    pub old_text: String,
    /// The replacement text
    pub new_text: String,
}

/// Messages sent from the Studio UI to the dev server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    /// Apply edits to a file
    EditFile {
        /// Path to the file (relative to project root)
        file: String,
        /// List of edits to apply
        edits: Vec<Edit>,
    },
    /// Undo the last edit to a file
    Undo {
        /// Path to the file
        file: String,
    },
    /// Redo the last undone edit to a file
    Redo {
        /// Path to the file
        file: String,
    },
    /// Request the current file content (for syncing)
    GetFile {
        /// Path to the file
        file: String,
    },
    /// Ping to keep connection alive
    Ping,
}

/// Messages sent from the dev server to the Studio UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    /// Result of an edit operation
    EditResult {
        /// Whether the edit succeeded
        success: bool,
        /// Error message if failed
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
    /// Notification that a file was updated (after successful edit or external change)
    FileUpdated {
        /// Path to the file
        file: String,
    },
    /// Response to GetFile request
    FileContent {
        /// Path to the file
        file: String,
        /// The file content
        content: String,
    },
    /// Undo/redo availability changed
    HistoryState {
        /// Path to the file
        file: String,
        /// Whether undo is available
        can_undo: bool,
        /// Whether redo is available
        can_redo: bool,
    },
    /// Response to Ping
    Pong,
    /// Error message for unexpected errors
    Error {
        /// Error description
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_message_serialization() {
        let msg = ClientMessage::EditFile {
            file: "src/routes/index.rs".to_string(),
            edits: vec![Edit {
                line: 42,
                old_text: "class=\"bg-blue-500\"".to_string(),
                new_text: "class=\"bg-red-500\"".to_string(),
            }],
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"edit_file\""));
        assert!(json.contains("\"line\":42"));

        // Round-trip
        let parsed: ClientMessage = serde_json::from_str(&json).unwrap();
        if let ClientMessage::EditFile { file, edits } = parsed {
            assert_eq!(file, "src/routes/index.rs");
            assert_eq!(edits.len(), 1);
            assert_eq!(edits[0].line, 42);
        } else {
            panic!("Expected EditFile");
        }
    }

    #[test]
    fn test_server_message_serialization() {
        let msg = ServerMessage::EditResult {
            success: true,
            error: None,
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"type\":\"edit_result\""));
        assert!(json.contains("\"success\":true"));
        // error should be omitted when None
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_server_message_with_error() {
        let msg = ServerMessage::EditResult {
            success: false,
            error: Some("File not found".to_string()),
        };

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("\"error\":\"File not found\""));
    }

    #[test]
    fn test_ping_pong() {
        let ping = ClientMessage::Ping;
        let json = serde_json::to_string(&ping).unwrap();
        assert_eq!(json, r#"{"type":"ping"}"#);

        let pong = ServerMessage::Pong;
        let json = serde_json::to_string(&pong).unwrap();
        assert_eq!(json, r#"{"type":"pong"}"#);
    }
}
