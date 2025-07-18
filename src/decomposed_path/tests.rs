// ---
// description: tests.rs for DecomposedPath
// ---

// ---

use super::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_from_path_simple_dir() -> Result<(), Box<dyn std::error::Error>> {
    // Note that /tmp  or /var are symlinked.
    let cache_base = std::env::var("XDG_CACHE_HOME")
        .or_else(|_| std::env::var("HOME").map(|home| format!("{}/.cache", home)))
        .expect("Neither XDG_CACHE_HOME nor HOME is set");

    let dir = tempfile::Builder::new()
        .prefix("gitignore-checker_")
        .tempdir_in(cache_base)?;
    let path = dir.path();

    let decomposed = DecomposedPath::from_path(path).unwrap();
    assert_eq!(decomposed.path_components.len(), path.components().count());
    assert!(decomposed.is_dir);

    for comp in decomposed.path_components {
        assert!(!comp.is_symlink);
    }

    Ok(())
}

#[test]
fn test_from_path_with_symlink() -> Result<(), Box<dyn std::error::Error>> {
    // Note that /tmp  or /var are symlinked.
    let cache_base = std::env::var("XDG_CACHE_HOME")
        .or_else(|_| std::env::var("HOME").map(|home| format!("{}/.cache", home)))
        .expect("Neither XDG_CACHE_HOME nor HOME is set");

    let dir = tempfile::Builder::new()
        .prefix("gitignore-checker_")
        .tempdir_in(cache_base)?;

    let target = dir.path().join("target");
    let link = dir.path().join("link");

    // Create target directory and symlink to it (Unix only)
    #[cfg(unix)]
    {
        fs::create_dir(&target).unwrap();
        std::os::unix::fs::symlink(&target, &link).unwrap();

        let decomposed = DecomposedPath::from_path(&link).unwrap();
        let last = decomposed.path_components.last().unwrap();

        assert_eq!(last.name, "link");
        assert!(last.is_symlink);
        assert!(decomposed.is_dir);
    }

    #[cfg(not(unix))]
    {
        // On Windows or unsupported platforms, just test basic decomposition without symlink
        fs::create_dir(&target).unwrap();
        let decomposed = DecomposedPath::from_path(&target).unwrap();
        assert!(!decomposed.path_components.last().unwrap().is_symlink);
    }

    Ok(())
}

#[test]
fn test_comp_is_dir_behavior() {
    let dir = tempdir().unwrap();
    let path = dir.path();

    let decomposed = DecomposedPath::from_path(path).unwrap();
    let last_index = decomposed.path_components.len() - 1;

    assert_eq!(decomposed.comp_is_dir(last_index), decomposed.is_dir);

    if last_index > 0 {
        assert!(decomposed.comp_is_dir(last_index - 1));
    }
}
