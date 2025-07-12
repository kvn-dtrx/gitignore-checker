// ---
// description: Tests for the module `rule`.
// ---

// ---

#[cfg(test)]
mod rule_tests {
    use super::super::Rule;
    use std::path::Path;

    #[test]
    fn test_universal_node_glob_pattern() {
        let rule = Rule::get_rule("*.log").unwrap();
        let path = Path::new("foo/bar/baz.log");
        assert!(rule.matches(path, false));
    }

    #[test]
    fn test_universal_dir_const_pattern() {
        let rule = Rule::get_rule("target/").unwrap();
        assert!(rule.is_directory);
        assert!(rule.matches(Path::new("target"), true));
        assert!(!rule.matches(Path::new("target/file.txt"), false));
    }

    #[test]
    fn test_anchored_path() {
        let rule = Rule::get_rule("/foo/bar").unwrap();
        assert!(rule.is_anchored);
        assert!(rule.matches(Path::new("foo/bar"), false));
        assert!(!rule.matches(Path::new("baz/foo/bar"), false));
    }
}
