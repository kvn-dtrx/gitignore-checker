// ---
// description: mod.rs for Rules (Plural!)
// ---

// ---

mod rule;

use crate::decomposed_path::DecomposedPath;
use std::io;

use log::debug;
use log::info;
use rule::Rule;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Rules {
    pub rules: Vec<Rule>,
}

impl Rules {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut rules = Self::new();

        for line_result in reader.lines() {
            let line = line_result?;
            if let Some(rule) = Rule::from_line(&line) {
                rules.add(rule);
            }
        }
        Ok(rules)
    }

    pub fn new() -> Self {
        Rules { rules: Vec::new() }
    }

    pub fn add(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn ignore(&self, path: &Path) -> bool {
        println!("{:?}", path);
        let decomposed_path = DecomposedPath::from_path(path).unwrap();
        self.ignore_(&decomposed_path)
    }

    pub fn ignore_(&self, decomposed_path: &DecomposedPath) -> bool {
        let mut current_path = PathBuf::new();
        let mut prev_ignored = false;
        let mut ignored = false;
        let components = &decomposed_path.path_components;

        info!("Checking the following path:\n  '{:?}'", decomposed_path);
        info!(
            "Checking against the following rules:\n  '{:?}'",
            self.rules
        );

        for (index, component) in components.iter().enumerate() {
            current_path.push(&component.name);
            let mut path_str = current_path.to_string_lossy().into_owned();
            let is_dir = decomposed_path.comp_is_dir(index);
            if is_dir {
                path_str = format!("{}/", path_str);
            }
            info!(
                "Parent Path against which rules are checked:\n  '{}'",
                path_str
            );
            for (i, rule) in self.rules.iter().enumerate() {
                let ipp = i + 1;
                let git_pattern = rule.git_pattern();
                info!("Checking against {}-th rule:\n  '{}'", ipp, git_pattern);
                if rule.matches(&path_str) {
                    debug!("Rule is matching.");
                    if rule.is_negating {
                        ignored = false;
                        if ignored == prev_ignored {
                            debug!("Rule is unignoring again.")
                        } else {
                            info!("Rule is unignoring.")
                        }
                    } else {
                        ignored = true;
                        if ignored == prev_ignored {
                            debug!("Rule is ignoring again.")
                        } else {
                            info!("Rule is ignoring.")
                        }
                    }
                } else {
                    debug!("Rule is not matching.");
                }
                prev_ignored = ignored;
                if component.is_symlink {
                    if index + 1 < components.len() {
                        info!(
                            "Parent Path is a symbolic link.\n\
                            Consequently, all proper subordinate paths shall be ignored."
                        );
                        ignored = true;
                    } else {
                        info!("Path itself represents a symbolic link.");
                    }
                    return ignored;
                }
            }

            if ignored && is_dir {
                info!(
                    "Parent path is an ignored directory.\n\
                    Consequently, all subordinate paths shall be ignored as well."
                );
                return true;
            } else {
                debug!("Parent Path is not an ignored directory.");
            }
        }
        info!(
            "Result:\n  Path: {:?}\n  Rules: {:?}\n  Ignored: {:?}",
            decomposed_path, self.rules, ignored
        );
        ignored
    }
}

#[cfg(test)]
mod tests;
