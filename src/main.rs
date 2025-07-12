// ---
// description:
// ---

// ---

use glob::Pattern;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::path::Path;

mod rule;
use rule::Rule;

mod nodeinfo;
// use rule::Rule;


// 'a ensures that &'a P occuring in target lives
// as long as &'a [P] in paths.
// P represents a generic type representing a single path.
// R is a generic type for rules.
// F is a generic type for function or closure types consuming
// (&P, &R) and returning bool
// fn filter<'a, P, R, F>(paths: &'a [P], rules: &R, is_ignored: F) -> Vec<&'a P>
// where
//     F: Fn(&P, &R) -> bool,
// {
//     paths
//         // Iterator over &P.
//         .iter()
//         // *path dereference &&P to &P.
//         .filter(|path| is_ignored(*path, rules))
//         // Back-Conversion to collection.
//         .collect()
// }

// fn build_globset(patterns: &[&str]) -> GlobSet {
//     let mut builder = GlobSetBuilder::new();
//     for pattern in patterns {
//         builder.add(Glob::new(pattern).expect("Invalid glob pattern"));
//     }
//     builder.build().expect("Failed to build globset")
// }

fn is_ignored(path_plus: PathPlus, rules: &[&str]) -> bool {
    // The default state of a path is to be not ignored.
    let mut is_ignored = false;
    // let mut is_ignored_prev = None;
    let mut ignored_dirs = Vec::new();
    for rule in rules {
        // Whether rule is matching.
        if path subordinated to rule.globpattern {
            // Rule is matching
            // Whether rule is unignoring or ignoring.
            if rule.is_negating {
                if path_plus not in ignored_dirs {
                    // Rule is unignoring.
                    is_ignored = false;
                } else {
                    
                }
                // if is_ignored !=  is_ignored_prev { info } else {debug}
            } else {
                // Rule is ignoring.
                // Whether rule terminally ignores a directory.
                if rule.isdir {
                    return true
                } else {
                    is_ignore = true;
                    // if is_ignored !=  is_ignored_prev { info } else {debug}
                }
            }
        } else {
            // Rule is not matching.
        }
        is_ignored_prev = Some(is_ignored);
    }
    is_ignored
}

fn main() {
    // let paths = vec!["src/main.rs", "target/debug", "README.md"];
    // let rules = vec!["target", "node_modules"];

    // let ignored_paths = filter(&paths, &rules, |path, rules| {
    //     rules.iter().any(|rule| path.contains(rule))
    // });

    // for path in ignored_paths {
    //     println!("Ignored: {}", path);
    // }
}
