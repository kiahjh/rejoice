//! WebSocket handler for Rejoice Studio.
//!
//! Handles the `/__studio` WebSocket endpoint for the Studio UI
//! to communicate with the dev server.

use super::file_ops::FileOps;
use super::protocol::{ClientMessage, ServerMessage};
use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};

/// Handle a Studio WebSocket connection.
///
/// This function processes incoming messages from the Studio UI,
/// dispatches them to the appropriate handlers, and sends responses.
pub async fn handle_studio_socket(
    socket: WebSocket,
    file_ops: Arc<FileOps>,
    mut reload_rx: broadcast::Receiver<&'static str>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Create a channel for sending messages to the client
    let (tx, mut rx) = mpsc::channel::<ServerMessage>(32);

    // Spawn a task to forward reload messages
    let tx_clone = tx.clone();
    let reload_task = tokio::spawn(async move {
        while let Ok(msg) = reload_rx.recv().await {
            let server_msg = ServerMessage::FileUpdated {
                file: msg.to_string(),
            };
            if tx_clone.send(server_msg).await.is_err() {
                break;
            }
        }
    });

    // Spawn a task to send messages to the WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Process incoming messages
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            let response = handle_message(&text, &file_ops);
            if tx.send(response).await.is_err() {
                break;
            }
        }
    }

    // Clean up
    reload_task.abort();
    send_task.abort();
}

/// Handle a single client message and return the appropriate response.
fn handle_message(text: &str, file_ops: &FileOps) -> ServerMessage {
    let client_msg: ClientMessage = match serde_json::from_str(text) {
        Ok(msg) => msg,
        Err(e) => {
            return ServerMessage::Error {
                message: format!("Invalid message format: {}", e),
            };
        }
    };

    match client_msg {
        ClientMessage::EditFile { file, edits } => {
            let result = file_ops.apply_edits(&file, &edits);
            ServerMessage::EditResult {
                success: result.success,
                error: result.error,
            }
        }
        ClientMessage::Undo { file } => {
            let result = file_ops.undo(&file);
            ServerMessage::EditResult {
                success: result.success,
                error: result.error,
            }
        }
        ClientMessage::Redo { file } => {
            let result = file_ops.redo(&file);
            ServerMessage::EditResult {
                success: result.success,
                error: result.error,
            }
        }
        ClientMessage::GetFile { file } => match file_ops.read_file(&file) {
            Ok(content) => ServerMessage::FileContent { file, content },
            Err(e) => ServerMessage::Error { message: e },
        },
        ClientMessage::Ping => ServerMessage::Pong,
    }
}

/// Get the history state for a file.
pub fn get_history_state(file_ops: &FileOps, file: &str) -> ServerMessage {
    ServerMessage::HistoryState {
        file: file.to_string(),
        can_undo: file_ops.can_undo(file),
        can_redo: file_ops.can_redo(file),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, FileOps) {
        let temp_dir = TempDir::new().unwrap();
        let file_ops = FileOps::new(temp_dir.path()).unwrap();
        (temp_dir, file_ops)
    }

    #[test]
    fn test_handle_ping() {
        let (_temp_dir, file_ops) = setup_test_env();
        let msg = r#"{"type":"ping"}"#;
        let response = handle_message(msg, &file_ops);

        assert!(matches!(response, ServerMessage::Pong));
    }

    #[test]
    fn test_handle_edit_file() {
        let (temp_dir, file_ops) = setup_test_env();

        // Create a test file
        let test_file = temp_dir.path().join("test.rs");
        std::fs::write(&test_file, "let x = 1;\n").unwrap();

        let msg = r#"{"type":"edit_file","file":"test.rs","edits":[{"line":1,"old_text":"1","new_text":"42"}]}"#;
        let response = handle_message(msg, &file_ops);

        if let ServerMessage::EditResult { success, error } = response {
            assert!(success);
            assert!(error.is_none());
        } else {
            panic!("Expected EditResult");
        }

        // Verify the file was changed
        let content = std::fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("42"));
    }

    #[test]
    fn test_handle_get_file() {
        let (temp_dir, file_ops) = setup_test_env();

        let test_file = temp_dir.path().join("test.rs");
        std::fs::write(&test_file, "hello world\n").unwrap();

        let msg = r#"{"type":"get_file","file":"test.rs"}"#;
        let response = handle_message(msg, &file_ops);

        if let ServerMessage::FileContent { file, content } = response {
            assert_eq!(file, "test.rs");
            assert!(content.contains("hello world"));
        } else {
            panic!("Expected FileContent");
        }
    }

    #[test]
    fn test_handle_invalid_message() {
        let (_temp_dir, file_ops) = setup_test_env();
        let msg = r#"{"type":"unknown_type"}"#;
        let response = handle_message(msg, &file_ops);

        assert!(matches!(response, ServerMessage::Error { .. }));
    }

    #[test]
    fn test_handle_undo() {
        let (temp_dir, file_ops) = setup_test_env();

        let test_file = temp_dir.path().join("test.rs");
        std::fs::write(&test_file, "let x = 1;\n").unwrap();

        // Apply an edit first
        let edit_msg = r#"{"type":"edit_file","file":"test.rs","edits":[{"line":1,"old_text":"1","new_text":"2"}]}"#;
        handle_message(edit_msg, &file_ops);

        // Now undo
        let undo_msg = r#"{"type":"undo","file":"test.rs"}"#;
        let response = handle_message(undo_msg, &file_ops);

        if let ServerMessage::EditResult { success, error } = response {
            assert!(success);
            assert!(error.is_none());
        } else {
            panic!("Expected EditResult");
        }

        // Verify original content restored
        let content = std::fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("let x = 1;"));
    }
}
