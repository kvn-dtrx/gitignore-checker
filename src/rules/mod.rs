// ---
// description: mod.rs for Rules (Plural!)
// ---

// ---

mod rule;

use crate::decomposed_path::DecomposedPath;
use std::io;

use log::{debug, info};
use rule::Rule;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

pub struct Rules {
    pub rules: Vec<Rule>,
}

impl Rules {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut rules = Vec::new();

        for line_result in reader.lines() {
            let line = line_result?;
            if let Some(rule) = Rule::from_line(&line) {
                rules.push(rule);
            }
        }
        Ok(Self { rules })
    }

    pub fn new() -> Self {
        Rules { rules: Vec::new() }
    }

    pub fn add(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn ignore(&self, path: &Path) -> bool {
        let decomposed_path = DecomposedPath::from_path(path).unwrap();
        let mut current_path = PathBuf::new();
        let mut prev_ignored = false;
        let mut ignored = false;
        let components = &decomposed_path.path_components;

        for (index, component) in components.iter().enumerate() {
            current_path.push(&component.name);
            let mut path_str = current_path.to_string_lossy().into_owned();
            if decomposed_path.comp_is_dir(index) {
                path_str = format!("{}/", path_str);
            }
            for rule in &self.rules {
                let git_pattern = rule.git_pattern();
                if rule.matches(&path_str) {
                    debug!("Rule is matching:\n  {}", git_pattern);
                    if rule.is_negating {
                        ignored = false;
                        if ignored == prev_ignored {
                            debug!("Rule is unignoring again:\n {}", git_pattern)
                        } else {
                            info!("Rule is unignoring:\n {}", git_pattern)
                        }
                    } else {
                        ignored = true;
                        if ignored == prev_ignored {
                            debug!("Rule is ignoring again:\n {}", git_pattern)
                        } else {
                            info!("Rule is ignoring:\n {}", git_pattern)
                        }
                    }
                }
                prev_ignored = ignored;
                if component.is_symlink {
                    if index + 1 < components.len() {
                        info!("Parent Path is a symbolic link:\n {}", path_str);
                        return ignored;
                    } else {
                        info!("Path is a symbolic link.");
                    }
                }
            }

            if ignored && path.is_dir() {
                return true;
            }
        }
        ignored
    }
}

#[cfg(test)]
mod tests;
