use crate::config::Config;
use crate::issue::Issue;
use crate::rule::{LintRule, TRAILING_WHITESPACE_RULE_NAME};

pub struct TrailingWhitespaceLint {}

impl LintRule for TrailingWhitespaceLint {
    fn name(&self) -> &'static str {
        TRAILING_WHITESPACE_RULE_NAME
    }

    fn check(&self, _config: &Config, filename: &str, content: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        for (i, line) in content.lines().enumerate() {
            if line.len() > 0 && line.trim_end() != line {
                issues.push(Issue::new(
                    TRAILING_WHITESPACE_RULE_NAME,
                    filename,
                    "Line contains trailing whitespace",
                    i + 1,
                ));
            }
        }

        issues
    }

    fn format(&self, config: &Config, content: &str) -> String {
        let line_ending = config.line_ending.get_ending(content);
        let mut formatted_lines = Vec::new();

        for line in content.lines() {
            if line.len() > 0 && line.trim_end() != line {
                formatted_lines.push(line.trim_end().to_string());
            } else {
                formatted_lines.push(line.to_string());
            }
        }

        let mut result = formatted_lines.join(line_ending);

        if !result.ends_with(line_ending) {
            result.push_str(line_ending);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::line::LineEnding;

    #[test]
    fn test_trailing_whitespace_lint_check_with_trailing_whitespace() {
        let config = Config::new(LineEnding::Unix);
        let lint = TrailingWhitespaceLint {};
        let content = "This is a test file.   \nThis line is fine.\n  ";
        let filename = "test_file.rs";

        let issues = lint.check(&config, filename, content);
        assert_eq!(
            issues.len(),
            2,
            "There should be two issues for trailing whitespace"
        );
        assert_eq!(
            issues[0].line_number, 1,
            "The first issue should be on line 1"
        );
        assert_eq!(
            issues[1].line_number, 3,
            "The second issue should be on line 3"
        );
    }

    #[test]
    fn test_trailing_whitespace_lint_check_no_trailing_whitespace() {
        let config = Config::new(LineEnding::Unix);
        let lint = TrailingWhitespaceLint {};
        let content = "This is a test file.\nThis line is fine.\n";

        let issues = lint.check(&config, "test_file.rs", content);
        assert!(
            issues.is_empty(),
            "There should be no issues when there is no trailing whitespace"
        );
    }

    #[test]
    fn test_trailing_whitespace_lint_format_removes_trailing_whitespace() {
        let config = Config::new(LineEnding::Unix);
        let lint = TrailingWhitespaceLint {};
        let content = "This is a test file.   \nThis line is fine.\n  ";

        let formatted = lint.format(&config, content);
        let expected = "This is a test file.\nThis line is fine.\n";
        assert_eq!(formatted, expected, "Trailing whitespace should be removed");
    }

    #[test]
    fn test_trailing_whitespace_lint_format_does_nothing_if_no_trailing_whitespace() {
        let config = Config::new(LineEnding::Unix);
        let lint = TrailingWhitespaceLint {};
        let content = "This is a test file.\nThis line is fine.\n";

        let formatted = lint.format(&config, content);
        assert_eq!(
            formatted, content,
            "The content should not be changed if there is no trailing whitespace"
        );
    }
}
