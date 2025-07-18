// ---
// description: tests.rs for Rules (Plural!)
// ---

// ---

// use super::*;
use crate::decomposed_path::PathComponent;
use crate::rules::Rule;
use crate::utils::init_logger;
use std::fs;
use tempfile::tempdir;

// Helper: create a Rule from a gitignore line string.
fn rule_from_line(line: &str) -> Rule {
    Rule::from_line(line).expect("Rule parsing failed")
}

#[test]
fn test_rules_new_and_add() {
    init_logger();
    let mut rules = Rules::new();
    assert_eq!(rules.rules.len(), 0);

    let rule = rule_from_line("*.log");
    rules.add(rule);
    assert_eq!(rules.rules.len(), 1);
}

#[test]
fn test_from_file_reads_rules() {
    init_logger();

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
    init_logger();

    let mut rules = Rules::new();
    rules.add(rule_from_line("ignored_dir/"));
    rules.add(rule_from_line("!ignored_dir/not_ignored.txt"));

    let dir = DecomposedPath {
        path_components: vec![PathComponent {
            name: "ignored_dir".to_string(),
            is_symlink: false,
        }],
        is_dir: true,
    };

    let not_ignored = DecomposedPath {
        path_components: vec![
            PathComponent {
                name: "ignored_dir".to_string(),
                is_symlink: false,
            },
            PathComponent {
                name: "not_ignored.txt".to_string(),
                is_symlink: false,
            },
        ],
        is_dir: false,
    };

    let ignored_file = DecomposedPath {
        path_components: vec![
            PathComponent {
                name: "ignored_dir".to_string(),
                is_symlink: false,
            },
            PathComponent {
                name: "ignored_file.txt".to_string(),
                is_symlink: false,
            },
        ],
        is_dir: false,
    };

    assert!(rules.ignore_(&dir));
    assert!(rules.ignore_(&not_ignored));
    assert!(rules.ignore_(&ignored_file));
}

#[test]
fn test_ignore_symlink_handling_unit() {
    init_logger();

    let mut rules = Rules::new();
    rules.add(rule_from_line("link/"));

    let link_dir = DecomposedPath {
        path_components: vec![PathComponent {
            name: "link".to_string(),
            is_symlink: true,
        }],
        is_dir: true,
    };

    let real_dir = DecomposedPath {
        path_components: vec![PathComponent {
            name: "real".to_string(),
            is_symlink: false,
        }],
        is_dir: true,
    };

    assert!(rules.ignore_(&link_dir));

    assert!(!rules.ignore_(&real_dir));
}

#[test]
fn test_foo() {
    init_logger();

    let mut rules = Rules::new();
    // rules.add(rule_from_line("bar"));
    // rules.add(rule_from_line("!*/"));
    rules.add(rule_from_line("!*.txt"));

    let txt = DecomposedPath {
        path_components: vec![
            PathComponent {
                name: "foo".to_string(),
                is_symlink: true,
            },
            PathComponent {
                name: "bar".to_string(),
                is_symlink: false,
            },
            // PathComponent {
            //     name: "baz.tex".to_string(),
            //     is_symlink: false,
            // },
        ],
        is_dir: false,
    };

    assert!(rules.ignore_(&txt));
}
