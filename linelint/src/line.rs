pub const UNIX_LINE_ENDING: &str = "\n";
pub const WINDOWS_LINE_ENDING: &str = "\r\n";

#[derive(Debug)]
pub enum LineEnding {
    Auto,
    Unix,
    Windows,
}

impl LineEnding {
    pub fn get_ending<'a>(&self, content: &'a str) -> &'a str {
        match self {
            LineEnding::Auto => detect_line_ending(content),
            LineEnding::Unix => UNIX_LINE_ENDING,
            LineEnding::Windows => WINDOWS_LINE_ENDING,
        }
    }
}

pub fn detect_line_ending(content: &str) -> &str {
    if content.contains("\r\n") {
        WINDOWS_LINE_ENDING
    } else {
        UNIX_LINE_ENDING
    }
}
