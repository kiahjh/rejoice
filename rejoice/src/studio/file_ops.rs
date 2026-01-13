//! File operations for Rejoice Studio.
//!
//! Provides editing, undo, and redo functionality with disk-based history.
//! History is stored in `.rejoice-studio/history/` to survive restarts.

use super::protocol::Edit;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Directory for Studio data
const STUDIO_DIR: &str = ".rejoice-studio";
/// Subdirectory for file history
const HISTORY_DIR: &str = "history";
/// Maximum number of backups to keep per file
const MAX_BACKUPS: usize = 20;

/// Manages file operations and history for Studio.
pub struct FileOps {
    /// Root directory of the project
    project_root: PathBuf,
    /// Path to the history directory
    history_dir: PathBuf,
}

/// Result of a file operation
#[derive(Debug)]
pub struct EditResult {
    pub success: bool,
    pub error: Option<String>,
    pub can_undo: bool,
    pub can_redo: bool,
}

impl FileOps {
    /// Create a new FileOps instance for the given project root.
    pub fn new(project_root: impl Into<PathBuf>) -> io::Result<Self> {
        let project_root = project_root.into();
        let studio_dir = project_root.join(STUDIO_DIR);
        let history_dir = studio_dir.join(HISTORY_DIR);

        // Create directories if they don't exist
        if !studio_dir.exists() {
            fs::create_dir_all(&studio_dir)?;
            // Create .gitignore in studio dir
            fs::write(studio_dir.join(".gitignore"), "*\n")?;
        }
        if !history_dir.exists() {
            fs::create_dir_all(&history_dir)?;
        }

        Ok(Self {
            project_root,
            history_dir,
        })
    }

    /// Apply edits to a file.
    ///
    /// Saves the current state to history before applying edits.
    pub fn apply_edits(&self, file: &str, edits: &[Edit]) -> EditResult {
        let file_path = self.project_root.join(file);

        // Read current content
        let content = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                return EditResult {
                    success: false,
                    error: Some(format!("Failed to read file: {}", e)),
                    can_undo: self.can_undo(file),
                    can_redo: self.can_redo(file),
                };
            }
        };

        // Save current state to history before editing
        if let Err(e) = self.save_to_history(file, &content) {
            return EditResult {
                success: false,
                error: Some(format!("Failed to save history: {}", e)),
                can_undo: self.can_undo(file),
                can_redo: self.can_redo(file),
            };
        }

        // Clear redo history since we're making a new edit
        self.clear_redo_history(file);

        // Apply edits
        let lines: Vec<&str> = content.lines().collect();
        let mut new_lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();

        for edit in edits {
            let line_idx = (edit.line as usize).saturating_sub(1);
            if line_idx >= new_lines.len() {
                return EditResult {
                    success: false,
                    error: Some(format!(
                        "Line {} out of range (file has {} lines)",
                        edit.line,
                        new_lines.len()
                    )),
                    can_undo: self.can_undo(file),
                    can_redo: self.can_redo(file),
                };
            }

            let line = &new_lines[line_idx];
            if !line.contains(&edit.old_text) {
                return EditResult {
                    success: false,
                    error: Some(format!(
                        "Text '{}' not found on line {}",
                        edit.old_text, edit.line
                    )),
                    can_undo: self.can_undo(file),
                    can_redo: self.can_redo(file),
                };
            }

            new_lines[line_idx] = line.replacen(&edit.old_text, &edit.new_text, 1);
        }

        // Write back
        let new_content = new_lines.join("\n");
        // Preserve trailing newline if original had one
        let new_content = if content.ends_with('\n') && !new_content.ends_with('\n') {
            new_content + "\n"
        } else {
            new_content
        };

        if let Err(e) = fs::write(&file_path, &new_content) {
            return EditResult {
                success: false,
                error: Some(format!("Failed to write file: {}", e)),
                can_undo: self.can_undo(file),
                can_redo: self.can_redo(file),
            };
        }

        EditResult {
            success: true,
            error: None,
            can_undo: self.can_undo(file),
            can_redo: self.can_redo(file),
        }
    }

    /// Undo the last edit to a file.
    pub fn undo(&self, file: &str) -> EditResult {
        let file_path = self.project_root.join(file);

        // Get the history directory for this file
        let file_history_dir = self.get_file_history_dir(file);
        if !file_history_dir.exists() {
            return EditResult {
                success: false,
                error: Some("No undo history available".to_string()),
                can_undo: false,
                can_redo: self.can_redo(file),
            };
        }

        // Find the latest backup
        let backups = match self.list_backups(&file_history_dir) {
            Ok(b) => b,
            Err(e) => {
                return EditResult {
                    success: false,
                    error: Some(format!("Failed to read history: {}", e)),
                    can_undo: false,
                    can_redo: self.can_redo(file),
                };
            }
        };

        if backups.is_empty() {
            return EditResult {
                success: false,
                error: Some("No undo history available".to_string()),
                can_undo: false,
                can_redo: self.can_redo(file),
            };
        }

        let latest_backup = &backups[backups.len() - 1];

        // Read current content and save to redo history
        if let Ok(current) = fs::read_to_string(&file_path) {
            let _ = self.save_to_redo_history(file, &current);
        }

        // Read the backup content
        let backup_content = match fs::read_to_string(latest_backup) {
            Ok(c) => c,
            Err(e) => {
                return EditResult {
                    success: false,
                    error: Some(format!("Failed to read backup: {}", e)),
                    can_undo: self.can_undo(file),
                    can_redo: self.can_redo(file),
                };
            }
        };

        // Write the backup content to the file
        if let Err(e) = fs::write(&file_path, &backup_content) {
            return EditResult {
                success: false,
                error: Some(format!("Failed to restore file: {}", e)),
                can_undo: self.can_undo(file),
                can_redo: self.can_redo(file),
            };
        }

        // Remove the used backup
        let _ = fs::remove_file(latest_backup);

        EditResult {
            success: true,
            error: None,
            can_undo: self.can_undo(file),
            can_redo: self.can_redo(file),
        }
    }

    /// Redo the last undone edit to a file.
    pub fn redo(&self, file: &str) -> EditResult {
        let file_path = self.project_root.join(file);

        // Get the redo directory for this file
        let redo_dir = self.get_file_redo_dir(file);
        if !redo_dir.exists() {
            return EditResult {
                success: false,
                error: Some("No redo history available".to_string()),
                can_undo: self.can_undo(file),
                can_redo: false,
            };
        }

        // Find the latest redo backup
        let backups = match self.list_backups(&redo_dir) {
            Ok(b) => b,
            Err(e) => {
                return EditResult {
                    success: false,
                    error: Some(format!("Failed to read redo history: {}", e)),
                    can_undo: self.can_undo(file),
                    can_redo: false,
                };
            }
        };

        if backups.is_empty() {
            return EditResult {
                success: false,
                error: Some("No redo history available".to_string()),
                can_undo: self.can_undo(file),
                can_redo: false,
            };
        }

        let latest_redo = &backups[backups.len() - 1];

        // Read current content and save to undo history
        if let Ok(current) = fs::read_to_string(&file_path) {
            let _ = self.save_to_history(file, &current);
        }

        // Read the redo content
        let redo_content = match fs::read_to_string(latest_redo) {
            Ok(c) => c,
            Err(e) => {
                return EditResult {
                    success: false,
                    error: Some(format!("Failed to read redo backup: {}", e)),
                    can_undo: self.can_undo(file),
                    can_redo: self.can_redo(file),
                };
            }
        };

        // Write the redo content to the file
        if let Err(e) = fs::write(&file_path, &redo_content) {
            return EditResult {
                success: false,
                error: Some(format!("Failed to restore file: {}", e)),
                can_undo: self.can_undo(file),
                can_redo: self.can_redo(file),
            };
        }

        // Remove the used redo backup
        let _ = fs::remove_file(latest_redo);

        EditResult {
            success: true,
            error: None,
            can_undo: self.can_undo(file),
            can_redo: self.can_redo(file),
        }
    }

    /// Read a file's content.
    pub fn read_file(&self, file: &str) -> Result<String, String> {
        let file_path = self.project_root.join(file);
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read file: {}", e))
    }

    /// Check if undo is available for a file.
    pub fn can_undo(&self, file: &str) -> bool {
        let file_history_dir = self.get_file_history_dir(file);
        if !file_history_dir.exists() {
            return false;
        }
        self.list_backups(&file_history_dir)
            .map(|b| !b.is_empty())
            .unwrap_or(false)
    }

    /// Check if redo is available for a file.
    pub fn can_redo(&self, file: &str) -> bool {
        let redo_dir = self.get_file_redo_dir(file);
        if !redo_dir.exists() {
            return false;
        }
        self.list_backups(&redo_dir)
            .map(|b| !b.is_empty())
            .unwrap_or(false)
    }

    /// Cleanup old history on startup (removes excess backups).
    pub fn cleanup_old_history(&self) -> io::Result<()> {
        if !self.history_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.history_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let dir_path = entry.path();
                let backups = self.list_backups(&dir_path)?;

                // Remove excess backups (keep only MAX_BACKUPS)
                if backups.len() > MAX_BACKUPS {
                    let to_remove = backups.len() - MAX_BACKUPS;
                    for backup in backups.iter().take(to_remove) {
                        let _ = fs::remove_file(backup);
                    }
                }
            }
        }

        Ok(())
    }

    // --- Private helpers ---

    fn get_file_history_dir(&self, file: &str) -> PathBuf {
        let encoded = encode_path(file);
        self.history_dir.join(encoded)
    }

    fn get_file_redo_dir(&self, file: &str) -> PathBuf {
        let encoded = encode_path(file);
        self.history_dir.join(format!("{}.redo", encoded))
    }

    fn save_to_history(&self, file: &str, content: &str) -> io::Result<()> {
        let file_history_dir = self.get_file_history_dir(file);
        fs::create_dir_all(&file_history_dir)?;

        let next_num = self.get_next_backup_number(&file_history_dir)?;
        let backup_path = file_history_dir.join(format!("{:04}.backup", next_num));
        fs::write(backup_path, content)?;

        // Cleanup if we have too many backups
        let backups = self.list_backups(&file_history_dir)?;
        if backups.len() > MAX_BACKUPS {
            let to_remove = backups.len() - MAX_BACKUPS;
            for backup in backups.iter().take(to_remove) {
                let _ = fs::remove_file(backup);
            }
        }

        Ok(())
    }

    fn save_to_redo_history(&self, file: &str, content: &str) -> io::Result<()> {
        let redo_dir = self.get_file_redo_dir(file);
        fs::create_dir_all(&redo_dir)?;

        let next_num = self.get_next_backup_number(&redo_dir)?;
        let backup_path = redo_dir.join(format!("{:04}.backup", next_num));
        fs::write(backup_path, content)?;

        Ok(())
    }

    fn clear_redo_history(&self, file: &str) {
        let redo_dir = self.get_file_redo_dir(file);
        if redo_dir.exists() {
            let _ = fs::remove_dir_all(&redo_dir);
        }
    }

    fn list_backups(&self, dir: &Path) -> io::Result<Vec<PathBuf>> {
        let mut backups: Vec<PathBuf> = fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("backup"))
            .collect();
        backups.sort();
        Ok(backups)
    }

    fn get_next_backup_number(&self, dir: &Path) -> io::Result<u32> {
        let backups = self.list_backups(dir)?;
        if backups.is_empty() {
            return Ok(1);
        }

        let last = &backups[backups.len() - 1];
        let stem = last.file_stem().and_then(|s| s.to_str()).unwrap_or("0000");
        let num: u32 = stem.parse().unwrap_or(0);
        Ok(num + 1)
    }
}

/// Encode a file path to be used as a directory name.
/// Replaces path separators with double underscores.
fn encode_path(path: &str) -> String {
    path.replace(['/', '\\'], "__")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_project() -> (TempDir, FileOps) {
        let temp_dir = TempDir::new().unwrap();
        let file_ops = FileOps::new(temp_dir.path()).unwrap();
        (temp_dir, file_ops)
    }

    #[test]
    fn test_apply_single_edit() {
        let (temp_dir, file_ops) = setup_test_project();

        // Create a test file
        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "let x = 1;\nlet y = 2;\n").unwrap();

        let result = file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 1,
                old_text: "1".to_string(),
                new_text: "42".to_string(),
            }],
        );

        assert!(result.success);
        assert!(result.error.is_none());

        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("let x = 42;"));
        assert!(content.contains("let y = 2;"));
    }

    #[test]
    fn test_apply_multiple_edits() {
        let (temp_dir, file_ops) = setup_test_project();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "let x = 1;\nlet y = 2;\n").unwrap();

        let result = file_ops.apply_edits(
            "test.rs",
            &[
                Edit {
                    line: 1,
                    old_text: "1".to_string(),
                    new_text: "10".to_string(),
                },
                Edit {
                    line: 2,
                    old_text: "2".to_string(),
                    new_text: "20".to_string(),
                },
            ],
        );

        assert!(result.success);

        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("let x = 10;"));
        assert!(content.contains("let y = 20;"));
    }

    #[test]
    fn test_edit_creates_history() {
        let (temp_dir, file_ops) = setup_test_project();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "let x = 1;\n").unwrap();

        file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 1,
                old_text: "1".to_string(),
                new_text: "2".to_string(),
            }],
        );

        assert!(file_ops.can_undo("test.rs"));
    }

    #[test]
    fn test_undo() {
        let (temp_dir, file_ops) = setup_test_project();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "let x = 1;\n").unwrap();

        // Apply edit
        file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 1,
                old_text: "1".to_string(),
                new_text: "2".to_string(),
            }],
        );

        // Verify edit was applied
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("let x = 2;"));

        // Undo
        let result = file_ops.undo("test.rs");
        assert!(result.success);

        // Verify undo restored original
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("let x = 1;"));
    }

    #[test]
    fn test_redo() {
        let (temp_dir, file_ops) = setup_test_project();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "let x = 1;\n").unwrap();

        // Apply edit
        file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 1,
                old_text: "1".to_string(),
                new_text: "2".to_string(),
            }],
        );

        // Undo
        file_ops.undo("test.rs");

        // Redo
        let result = file_ops.redo("test.rs");
        assert!(result.success);

        // Verify redo reapplied edit
        let content = fs::read_to_string(&test_file).unwrap();
        assert!(content.contains("let x = 2;"));
    }

    #[test]
    fn test_edit_clears_redo() {
        let (temp_dir, file_ops) = setup_test_project();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "let x = 1;\n").unwrap();

        // Apply edit, undo
        file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 1,
                old_text: "1".to_string(),
                new_text: "2".to_string(),
            }],
        );
        file_ops.undo("test.rs");

        assert!(file_ops.can_redo("test.rs"));

        // Apply new edit (should clear redo)
        file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 1,
                old_text: "1".to_string(),
                new_text: "3".to_string(),
            }],
        );

        assert!(!file_ops.can_redo("test.rs"));
    }

    #[test]
    fn test_edit_nonexistent_file() {
        let (_temp_dir, file_ops) = setup_test_project();

        let result = file_ops.apply_edits(
            "nonexistent.rs",
            &[Edit {
                line: 1,
                old_text: "x".to_string(),
                new_text: "y".to_string(),
            }],
        );

        assert!(!result.success);
        assert!(result.error.unwrap().contains("Failed to read file"));
    }

    #[test]
    fn test_edit_line_out_of_range() {
        let (temp_dir, file_ops) = setup_test_project();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "line 1\n").unwrap();

        let result = file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 100,
                old_text: "x".to_string(),
                new_text: "y".to_string(),
            }],
        );

        assert!(!result.success);
        assert!(result.error.unwrap().contains("out of range"));
    }

    #[test]
    fn test_edit_text_not_found() {
        let (temp_dir, file_ops) = setup_test_project();

        let test_file = temp_dir.path().join("test.rs");
        fs::write(&test_file, "let x = 1;\n").unwrap();

        let result = file_ops.apply_edits(
            "test.rs",
            &[Edit {
                line: 1,
                old_text: "not_found".to_string(),
                new_text: "y".to_string(),
            }],
        );

        assert!(!result.success);
        assert!(result.error.unwrap().contains("not found on line"));
    }

    #[test]
    fn test_encode_path() {
        assert_eq!(encode_path("src/routes/index.rs"), "src__routes__index.rs");
        assert_eq!(encode_path("simple.rs"), "simple.rs");
    }
}
