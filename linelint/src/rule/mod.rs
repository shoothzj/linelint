use crate::config::Config;
use crate::issue::Issue;

pub mod line_end_lint;
pub mod trailing_ws_lint;

pub const LINE_END_RULE_NAME: &str = "LineEnd";
pub const TRAILING_WHITESPACE_RULE_NAME: &str = "TrailingWhitespace";

pub const GOLANG_EMPTY_LINE_AFTER_FUNCTION_DEFINITION_RULE_NAME: &str =
    "GolangEmptyLineAfterFunctionDefinition";

pub const JAVA_EMPTY_LINE_AFTER_FUNCTION_DEFINITION_RULE_NAME: &str =
    "JavaEmptyLineAfterFunctionDefinition";

pub trait LintRule {
    fn name(&self) -> &'static str;

    fn check(&self, config: &Config, filename: &str, content: &str) -> Vec<Issue>;

    fn format(&self, config: &Config, content: &str) -> String;
}
