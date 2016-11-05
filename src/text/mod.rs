use std::convert::From;
use std::fmt;
use std::ops::{Add, Deref};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
pub struct Text {
    text: String,
}

pub trait Textable {
    fn to_text(self) -> Text;
}

// impl<T> Textable for T where String: From<T> {
//     fn to_text(self) -> Text {
//         Text { text: String::from(self) }
//     }
// }

impl<'a> Textable for &'a str {
    fn to_text(self) -> Text {
        Text { text: self.to_owned() }
    }
}

impl Textable for String {
    fn to_text(self) -> Text {
        Text { text: self }
    }
}

impl<'a> Textable for &'a Path {
    fn to_text(self) -> Text {
        Text { text: self.to_string_lossy().into_owned() }
    }
}

impl Textable for PathBuf {
    fn to_text(self) -> Text {
        Text { text: self.to_string_lossy().into_owned() }
    }
}

impl Text {
    pub fn new() -> Self {
        Text { text: String::new() }
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
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

/// Add

impl Add for Text {
    type Output = Text;
    fn add(self, rhs: Text) -> Self::Output {
        Text { text: self.text + &rhs.text }
    }
}

impl<T> Add<T> for Text
    where T: Textable
{
    type Output = Text;
    fn add(self, rhs: T) -> Self::Output {
        Text { text: self.text + &rhs.to_text() }
    }
}

/// PartialEq

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        *self == Text::from(other)
    }
}

impl<'a> PartialEq<&'a str> for Text {
    fn eq(&self, other: &&'a str) -> bool {
        self.text == *other
    }
}

impl PartialEq<String> for Text {
    fn eq(&self, other: &String) -> bool {
        &self.text == other
    }
}

impl<'a> PartialEq<String> for &'a Text {
    fn eq(&self, other: &String) -> bool {
        self.text == *other
    }
}

impl PartialEq<Path> for Text {
    fn eq(&self, other: &Path) -> bool {
        match other.to_str() {
            Some(s) => self.text == s,
            None => false,
        }
    }
}

impl<'a> PartialEq<&'a Path> for Text {
    fn eq(&self, other: &&'a Path) -> bool {
        match other.to_str() {
            Some(s) => self.text == s,
            None => false,
        }
    }
}

impl PartialEq<PathBuf> for Text {
    fn eq(&self, other: &PathBuf) -> bool {
        match other.to_str() {
            Some(s) => self.text == s,
            None => false,
        }
    }
}

impl<'a> PartialEq<PathBuf> for &'a Text {
    fn eq(&self, other: &PathBuf) -> bool {
        match other.to_str() {
            Some(s) => self.text == s,
            None => false,
        }
    }
}

impl<T> From<T> for Text
    where T: Textable
{
    fn from(s: T) -> Self {
        s.to_text()
    }
}

/// From<Text> for String

impl From<Text> for String {
    fn from(t: Text) -> Self {
        t.text
    }
}

impl<'a> From<&'a Text> for &'a Path {
    fn from(t: &'a Text) -> Self {
        Path::new(&t.text)
    }
}
