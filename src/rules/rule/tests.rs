// ---
// description: tests.rs for Rule (Singular!)
// ---

// ---

use crate::rules::Rule;

#[test]
fn test_rule_from_line_basic_pattern() {
    let rule = Rule::from_line("*.log").unwrap();
    assert_eq!(rule.reduced_pattern, "*.log");
    assert!(!rule.is_negating);
    assert!(!rule.is_directory);
    assert!(!rule.is_absolute);
}

#[test]
fn test_rule_from_line_negating_directory_absolute() {
    let rule = Rule::from_line("!/target/").unwrap();
    assert_eq!(rule.reduced_pattern, "target");
    assert!(rule.is_negating);
    assert!(rule.is_directory);
    assert!(rule.is_absolute); // because of the internal '/'
}

#[test]
fn test_rule_from_line_skips_comments_and_blank_lines() {
    assert!(Rule::from_line("# comment").is_none());
    assert!(Rule::from_line("   ").is_none());
}

#[test]
fn test_git_pattern_variants() {
    let r1 = Rule::from_line("foo").unwrap();
    assert_eq!(r1.git_pattern(), "foo");

    let r2 = Rule::from_line("foo/").unwrap();
    assert_eq!(r2.git_pattern(), "foo/");

    let r3 = Rule::from_line("/foo").unwrap();
    assert_eq!(r3.git_pattern(), "/foo");

    let r4 = Rule::from_line("/foo/").unwrap();
    assert_eq!(r4.git_pattern(), "/foo/");
}

#[test]
fn test_matches_with_glob() {
    let r = Rule::from_line("*.log").unwrap();
    assert!(r.matches("foo.log"));
    assert!(r.matches("build/foo.log"));
    assert!(!r.matches("foo.txt"));

    let r2 = Rule::from_line("target/").unwrap();
    assert!(r2.matches("foo/target/"));
    assert!(r2.matches("target/"));
    assert!(!r2.matches("target/file.rs"));
}
