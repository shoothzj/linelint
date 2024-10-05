use crate::line::LineEnding;
use crate::rule::LintRule;

pub struct Config {
    pub line_ending: LineEnding,
    pub rules: Vec<Box<dyn LintRule>>,
}

impl Config {
    pub fn new(line_ending: LineEnding) -> Self {
        Config {
            line_ending,
            rules: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Box<dyn LintRule>) {
        self.rules.push(rule);
    }
}
