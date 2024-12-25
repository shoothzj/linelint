use crate::config::Config;
use crate::issue::Issue;
use crate::line::LineEnding;
use ignore::WalkBuilder;
use std::path::Path;
use std::{fs, io};

pub struct Linter<'a> {
    config: &'a Config,
}

impl<'a> Linter<'a> {
    pub fn new(config: &'a Config) -> Self {
        Linter { config }
    }

    pub fn check_files_in_dir(
        &self,
        dir: &Path,
    ) -> Result<Vec<Issue>, Vec<(std::path::PathBuf, io::Error)>> {
        let mut all_issues = Vec::new();

        let result = self.travel_dir(dir, |path| match fs::read_to_string(path) {
            Ok(content) => {
                let issues = self.check(path.to_str().unwrap(), &content);
                all_issues.extend(issues);
                Ok(())
            }
            Err(e) => Err(e),
        });

        result.map(|_| all_issues)
    }

    pub fn format_files_in_dir(
        &self,
        dir: &Path,
    ) -> Result<(), Vec<(std::path::PathBuf, io::Error)>> {
        self.travel_dir(dir, |path| match fs::read_to_string(path) {
            Ok(content) => {
                let formatted_content = self.format(&content);
                if formatted_content != content {
                    fs::write(path, formatted_content).map_err(|e| e)
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e),
        })
    }

    fn travel_dir<F>(
        &self,
        dir: &Path,
        mut file_handler: F,
    ) -> Result<(), Vec<(std::path::PathBuf, io::Error)>>
    where
        F: FnMut(&Path) -> Result<(), io::Error>,
    {
        let mut errors = Vec::new();

        let mut builder = WalkBuilder::new(dir);

        builder
            .ignore(false)
            .hidden(false)
            .follow_links(true)
            .parents(true)
            .require_git(false)
            .git_exclude(true)
            .git_global(true)
            .git_ignore(true);

        builder.add_ignore(".linelintignore");

        let walker = builder.build();

        for entry in walker {
            match entry {
                Ok(entry) => {
                    if entry
                        .path()
                        .components()
                        .any(|comp| comp.as_os_str() == ".git")
                    {
                        continue;
                    }

                    if entry.path().file_name() == Some(std::ffi::OsStr::new(".gitmodules")) {
                        continue;
                    }

                    let ignored_extensions = ["xlsx", "xlss"];

                    if entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map_or(false, |ext| ignored_extensions.contains(&ext))
                    {
                        continue;
                    }

                    if let Ok(content) = fs::read(entry.path()) {
                        if std::str::from_utf8(&content).is_err() {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        if let Err(e) = file_handler(entry.path()) {
                            errors.push((entry.path().to_path_buf(), e));
                        }
                    }
                }
                Err(e) => errors.push((dir.to_path_buf(), io::Error::new(io::ErrorKind::Other, e))),
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn check(&self, filename: &str, content: &str) -> Vec<Issue> {
        let mut all_issues = Vec::new();
        for rule in &self.config.rules {
            let issues = rule.check(self.config, filename, content);
            all_issues.extend(issues);
        }
        all_issues
    }

    pub fn format(&self, content: &str) -> String {
        let mut formatted_content = content.to_string();
        for rule in &self.config.rules {
            formatted_content = rule.format(self.config, &formatted_content);
        }
        formatted_content
    }

    pub fn get_line_ending(&self) -> &LineEnding {
        &self.config.line_ending
    }
}
