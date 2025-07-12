// ---
// description: Tests for the module `rule`.
// ---

// ---

#[cfg(test)]
mod nodeinfo_tests {
    use super::super::EntryType;
    use super::super::NodeInfo;

    use std::fs;
    // For symlinks on Unix systems.
    use std::os::unix::fs as unix_fs;
    use tempfile::{NamedTempFile, tempdir};

    #[test]
    fn test_directory_detection() -> std::io::Result<()> {
        let dir = tempdir()?;
        let pi = NodeInfo::get_entry_type(dir.path())?;
        assert_eq!(pi.entry_type, EntryType::Directory);
        assert!(!pi.normalized_path.ends_with('/'));
        Ok(())
    }

    #[test]
    fn test_file_detection() -> std::io::Result<()> {
        let file = NamedTempFile::new()?;
        let pi = NodeInfo::get_entry_type(file.path())?;
        assert_eq!(pi.entry_type, EntryType::File);
        Ok(())
    }

    #[test]
    #[cfg(unix)]
    fn test_symlink_detection_unix() -> std::io::Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("target_file");
        fs::write(&target, b"hello")?;

        let link = dir.path().join("symlink");
        unix_fs::symlink(&target, &link)?;

        let pi = NodeInfo::get_entry_type(&link)?;
        assert_eq!(pi.entry_type, EntryType::Symlink);
        Ok(())
    }
}
