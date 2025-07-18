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

    pub fn glob_patterns(&self) -> Vec<String> {
        // This vector shall incorporate the derived glob patterns—
        // at least one, at most two.
        let mut glob_patterns = Vec::new();
        let mut glob_pattern = self.reduced_pattern.clone();
        // If the rule is relative, prepend a double glob star
        // to the glob pattern.
        if !self.is_absolute {
            glob_pattern = format!("**/{}", glob_pattern);
        }
        // If the rule is not directory-specific, add the glob
        // pattern as is to the vector.
        if !self.is_directory {
            glob_patterns.push(glob_pattern.clone());
        }
        // No matter whether the rule is directory-specific or not,
        // append a slash to the glob pattern and add the result
        // to the vector.
        glob_pattern = format!("{}/", glob_pattern);
        glob_patterns.push(glob_pattern.clone());
        // Now, the vector is fully populated.
        glob_patterns
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
        self.glob_patterns().iter().any(|pattern_str| {
            Pattern::new(pattern_str)
                .map(|p| p.matches(path))
                .unwrap_or(false)
        })
    }
}

#[cfg(test)]
mod tests;
