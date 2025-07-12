// ---
// description:
// ---

// ---

use glob::Pattern;
use std::path::Path;

// Represents a single `.gitignore` rule.
#[derive(Debug)]
pub struct Rule {
    // Cleaned pattern from the .gitignore file with no leading `!` or
    // trailing `/` (e.g., `*.log`, `**/target` or `[!0-9]xyz?`).
    pub git_pattern: Pattern,

    // Whether this rule negates a previous match (i.e., starts with `!`)
    pub is_negation: bool,

    // Whether this rule applies only to directories (i.e., ends with `/`)
    pub is_directory: bool,
}

impl Rule {
    fn get_rule(line: &str) -> Option<Rule> {
        // Removes leading/trailing white spaces from the line.
        let mut remaining_line = line.trim();

        // Skips comments.
        if remaining_line.starts_with('#') {
            return None;
        }

        // A prepended exclamation mark indicates negation.
        let is_negation = remaining_line.starts_with('!');
        remaining_line = remaining_line.strip_prefix('!');

        // An appended slash represents directories
        let is_directory = remaining_line.ends_with('/');
        remaining_line = remaining_line.strip_suffix('!');

        // Any further slash induces git to consider the
        // remaining as anchored.
        let is_anchored = remaining_line.contains('/');
        // Then, the information of leading slashes is
        // already incorporated.
        remaining_line = remaining_line.strip_prefix('/');

        let git_pattern = match Pattern::new(&remaining_line) {
            Ok(pat) => pat,
            Err(_) => return None,
        };

        Some(Self {
            git_pattern,
            is_negation,
            is_directory,
        })
    }

    fn get_glob_pattern(&self) -> Result<Pattern, glob::PatternError> {
        git_pattern = self.git_pattern;
        let pattern = if is_anchored {
            // ???
            format!("{}", remaining_line)
        } else {
            format!("**/{}", remaining_line)
        };
        Pattern::new(&pattern)
    }

    pub fn matches(&self, path: &Path, is_dir: bool) -> bool {
        if self.is_directory && !is_dir {
            return false;
        }

        let path_str = path
            .to_string_lossy()
            .replace('\\', "/")
            .trim_start_matches("./")
            .to_string();
        self.git_pattern.matches(&path_str)
    }
}

#[cfg(test)]
mod tests;
