// ---
// description: mod.rs for DecomposedPath
// ---

// ---

use std::fs;
use std::path::{Path, PathBuf};

pub struct PathComponent {
    pub name: String,
    pub is_symlink: bool,
}

pub struct DecomposedPath {
    pub path_components: Vec<PathComponent>,
    pub is_dir: bool,
}

impl DecomposedPath {
    // The relationship between Path and PathBuf is similar to that
    // of str and String: Particularly, there are no Path standalone variables.
    pub fn from_path(path: &Path) -> std::io::Result<Self> {
        let mut current_path = PathBuf::new();
        let mut path_components = Vec::new();

        for component in path.components() {
            let name = component.as_os_str().to_string_lossy().into_owned();
            current_path.push(&name);

            let is_symlink = fs::symlink_metadata(&current_path)
                .ok()
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false);

            path_components.push(PathComponent { name, is_symlink });
        }

        let is_dir = std::fs::symlink_metadata(&current_path)
            .ok()
            .map(|m| m.file_type().is_dir())
            .unwrap_or(false);

        Ok(DecomposedPath {
            path_components,
            is_dir,
        })
    }

    pub fn comp_is_dir(&self, index: usize) -> bool {
        match index == self.path_components.len() - 1 {
            true => self.is_dir,
            false => true,
        }
    }
}

#[cfg(test)]
mod tests;
