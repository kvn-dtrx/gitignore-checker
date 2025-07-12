// ---
// description: mod.rs for nodeinfo
// ---

// ---

use std::fs;
use std::path::{Path, PathBuf};

/// Represents the type of a filesystem entry relevant to `.gitignore` matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
    Symlink,
}

/// Holds a normalized path and its file type.
#[derive(Debug, Clone)]
pub struct NodeInfo {
    // Normalised means: Slashes instead of backslashes and no trailing backslashes.
    pub normalised_path: String,
    pub entry_type: EntryType,
}

impl NodeInfo {
    /// Constructs a new PathInfo from a Path reference by querying the filesystem.
    pub fn get_entry_type(path: &Path) -> std::io::Result<Self> {
        let metadata = fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();

        let entry_type = if file_type.is_dir() {
            EntryType::Directory
        } else if file_type.is_file() {
            EntryType::File
        } else if file_type.is_symlink() {
            EntryType::Symlink
        } else {
            // Fallback: treat unknown as File.
            EntryType::File
        };

        let normalised_path = path
            .components()
            .collect::<PathBuf>()
            .to_string_lossy()
            .replace('\\', "/");

        Ok(NodeInfo {
            normalised_path,
            entry_type,
        })
    }

    // Checks if this path info corresponds to a directory
    pub fn is_directory(&self) -> bool {
        self.entry_type == EntryType::Directory
    }

    // Checks if this path info corresponds to a directory
    pub fn is_symlink(&self) -> bool {
        self.entry_type == EntryType::Symlink
    }
}

#[cfg(test)]
mod tests;
