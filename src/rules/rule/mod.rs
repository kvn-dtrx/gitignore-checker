// ---
// description: mod.rs for Rule (Singular!)
// ---

// ---

use glob::Pattern;

// Represents a single `.gitignore` rule.
#[derive(Debug)]
pub struct Rule {
    // Glob pattern derived from git pattern; leading `!` and
    // and leading or trailing `/` are removed.
    // (e.g., `*.log`, `**/target` or `[!0-9]xyz?`).
    pub reduced_pattern: String,

    // Whether this rule negates a previous match (i.e., starts with `!`).
    pub is_negating: bool,

    // Whether this rule applies only to directories (i.e., ends with `/`).
    pub is_directory: bool,

    // Whether this rule shall be interpreted as absolute (i.e., starts with `/` after removal of a possible negation).
    pub is_absolute: bool,
}

impl Rule {
    pub fn from_line(line: &str) -> Option<Rule> {
        // Removes leading/trailing white spaces from the line.
        let mut remaining_line = line.trim();

        // Skip comments or empty lines.
        if remaining_line.is_empty() || remaining_line.starts_with('#') {
            return None;
        }

        // A prepended exclamation mark indicates negation.
        let is_negating = remaining_line.starts_with('!');
        remaining_line = remaining_line.strip_prefix('!').unwrap_or(remaining_line);

        // An appended slash represents directories
        let is_directory = remaining_line.ends_with('/');
        remaining_line = remaining_line.strip_suffix('/').unwrap_or(remaining_line);

        // Any further slash induces git to consider the
        // remaining line as absolute.
        let is_absolute = remaining_line.contains('/');
        // Then, the information of leading slashes is
        // already incorporated.
        remaining_line = remaining_line.strip_prefix('/').unwrap_or(remaining_line);

        let reduced_pattern = remaining_line.to_owned();

        Some(Self {
            reduced_pattern,
            is_negating,
            is_directory,
            is_absolute,
        })
    }

    pub fn glob_pattern(&self) -> String {
        let mut tmp_pattern = self.reduced_pattern.clone();
        if !self.is_absolute {
            // An relative reduced_pattern is prepended by a double glob star.
            tmp_pattern = format!("**/{}", tmp_pattern);
        }
        if self.is_directory {
            tmp_pattern = format!("{}/", tmp_pattern);
        }
        tmp_pattern
    }

    pub fn git_pattern(&self) -> String {
        let mut tmp_pattern = self.reduced_pattern.clone();
        if self.is_absolute {
            tmp_pattern = format!("/{}", tmp_pattern)
        };
        if self.is_directory {
            tmp_pattern = format!("{}/", tmp_pattern)
        };
        tmp_pattern
    }

    pub fn matches(&self, path: &str) -> bool {
        let glob_pattern_ = self.glob_pattern();
        let glob_pattern = Pattern::new(&glob_pattern_).unwrap();
        glob_pattern.matches(path)
    }
}

#[cfg(test)]
mod tests;
