use std::convert::From;
use std::fmt;
use std::ops::Deref;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new() -> Self {
        Text { text: String::new() }
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&self.text)
    }
}

impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &str {
        &self.text
    }
}

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        self.text == other
    }
}

impl<'a> From<&'a str> for Text {
    fn from(s: &'a str) -> Self {
        Text { text: s.to_owned() }
    }
}

impl From<String> for Text {
    fn from(s: String) -> Self {
        Text { text: s }
    }
}

impl<'a> From<&'a Path> for Text {
    fn from(s: &'a Path) -> Self {
        Text { text: s.to_string_lossy().into_owned() }
    }
}

impl From<PathBuf> for Text {
    fn from(s: PathBuf) -> Self {
        Text { text: s.to_string_lossy().into_owned() }
    }
}

impl From<Text> for String {
    fn from(t: Text) -> Self {
        t.text
    }
}
