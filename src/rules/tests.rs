// ---
// description: tests.rs for Rules (Plural!)
// ---

// ---

use super::*;
use crate::rules::Rule;
use std::fs::{self, File};
use std::path::PathBuf;
use tempfile::tempdir;

// Helper: create a Rule from a gitignore line string.
fn rule_from_line(line: &str) -> Rule {
    Rule::from_line(line).expect("Rule parsing failed")
}

#[test]
fn test_rules_new_and_add() {
    let mut rules = Rules::new();
    assert_eq!(rules.rules.len(), 0);

    let rule = rule_from_line("*.log");
    rules.add(rule);
    assert_eq!(rules.rules.len(), 1);
}

#[test]
fn test_from_file_reads_rules() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("gitignore");
    fs::write(&file_path, "*.log\n!important.log\n").unwrap();

    let rules = Rules::from_file(&file_path).unwrap();
    assert_eq!(rules.rules.len(), 2);
    assert!(rules.rules.iter().any(|r| r.reduced_pattern == "*.log"));
    assert!(rules.rules.iter().any(|r| r.is_negating));
}

#[test]
fn test_ignore_basic_patterns() {
    let mut rules = Rules::new();
    rules.add(rule_from_line("ignored_dir/"));
    rules.add(rule_from_line("!ignored_dir/not_ignored.txt"));

    let dir = tempdir().unwrap();

    let ignored_dir = dir.path().join("ignored_dir");
    fs::create_dir(&ignored_dir).unwrap();

    let not_ignored = ignored_dir.join("not_ignored.txt");
    File::create(&not_ignored).unwrap();

    let ignored_file = ignored_dir.join("ignored_file.txt");
    File::create(&ignored_file).unwrap();

    // The directory itself should be ignored.
    assert!(rules.ignore(&ignored_dir));

    // // The explicitly negated file should not be ignored.
    // assert!(!rules.ignore(&not_ignored));

    // // The other file should be ignored.
    // assert!(rules.ignore(&ignored_file));
}

#[test]
fn test_ignore_symlink_handling() {
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let dir = tempdir().unwrap();

        let real_dir = dir.path().join("real");
        fs::create_dir(&real_dir).unwrap();

        let link_dir = dir.path().join("link");
        symlink(&real_dir, &link_dir).unwrap();

        let mut rules = Rules::new();
        rules.add(rule_from_line("link/"));

        // The symlinked directory should be matched by the rule.
        assert!(rules.ignore(&link_dir));
    }
}
