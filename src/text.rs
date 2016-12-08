use std::convert::From;
use std::fmt;
use std::ops::{Add, AddAssign, Deref};
use std::ffi::{CString, OsString};
use std::path::{Path, PathBuf};

use super::content::{Content, ToContent};

#[derive(Debug, Default)]
pub struct Text {
    text: Vec<Content>,
}

// impl From<String> for Text {
//     fn from(s: String) -> Self {
//         Text { text: vec![Content::String(s)] }
//     }
// }

impl<T> From<T> for Text
    where T: ToContent<T>
{
    fn from(s: T) -> Self {
        Text { text: vec![Content::from(s)] }
    }
}

// impl From<OsString> for Text {
//     fn from(s: OsString) -> Self {
//         Text { text: vec![Content::OsString(s)] }
//     }
// }
//
// impl From<CString> for Text {
//     fn from(s: CString) -> Self {
//         Text { text: vec![Content::CString(s)] }
//     }
// }
//
// impl<T: Into<PathBuf>> From<T> for Text {
//     fn from(s: PathBuf) -> Self {
//         Text { text: vec![Content::PathBuf(s)] }
//     }
// }

// pub trait Textable<T> {
//     fn to_text(&self) -> Text;
// }
//
// impl<T> Textable<T> for T
//     where T: ToContent<T>
// {
//     fn to_text(&self) -> Text {
//         Text::from(self)
//     }
// }

// impl<'a> Textable for &'a str {
//     fn to_text(self) -> Text {
//         Text { text: self.to_owned() }
//     }
// }
//
// impl Textable for String {
//     fn to_text(self) -> Text {
//         Text { text: self }
//     }
// }
//
// impl<'a> Textable for &'a Path {
//     fn to_text(self) -> Text {
//         Text { text: self.to_string_lossy().into_owned() }
//     }
// }
//
// impl Textable for PathBuf {
//     fn to_text(self) -> Text {
//         Text { text: self.to_string_lossy().into_owned() }
//     }
// }

impl Text {
    pub fn new() -> Self {
        Text::default()
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        for elt in &self.text {
            len += elt.len();
        }
        len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{:?}", &self.text))
    }
}

impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &str {
        if self.is_empty() {
            ""
        } else {
            match self.text[0] {
                Content::String(ref s) => s,
                Content::OsString(ref s) => s.to_str().unwrap_or(""),
                Content::CString(ref s) => s.to_str().unwrap_or(""),
                Content::PathBuf(ref p) => p.to_str().unwrap_or(""),
            }
        }
    }
}

// Doesn't work - conflicting Deref implementation

// impl Deref for Text {
//     type Target = Path;
//     fn deref(&self) -> &Path {
//         Path::new(&self.text)
//     }
// }

// Add

impl Add for Text {
    type Output = Text;
    fn add(self, rhs: Text) -> Self::Output {
        let mut text = self.text.clone();
        text.extend_from_slice(&rhs.text);
        Text { text: text }
    }
}


// impl<T> Add<T> for Text
//     where T: Textable
// {
//     type Output = Text;
//     fn add(self, rhs: T) -> Self::Output {
//         Text { text: self.text + &rhs.to_text() }
//     }
// }

// AddAssign implementation

impl AddAssign for Text {
    fn add_assign(&mut self, other: Self) {
        self.text.extend_from_slice(&other.text)
    }
}

// PartialEq implementation

impl<T: ?Sized + AsRef<str>> PartialEq<T> for Text {
    default fn eq(&self, other: &T) -> bool {
        unimplemented!()
    }
}

// impl PartialEq<str> for Text {
//     fn eq(&self, other: &str) -> bool {
//         *self == Text::from(other)
//     }
// }
//
// impl<'a> PartialEq<&'a str> for Text {
//     fn eq(&self, other: &&'a str) -> bool {
//         self.text == *other
//     }
// }
//
// impl PartialEq<String> for Text {
//     fn eq(&self, other: &String) -> bool {
//         &self.text == other
//     }
// }
//
// impl<'a> PartialEq<String> for &'a Text {
//     fn eq(&self, other: &String) -> bool {
//         self.text == *other
//     }
// }
//
// impl PartialEq<Path> for Text {
//     fn eq(&self, other: &Path) -> bool {
//         match other.to_str() {
//             Some(s) => self.text == s,
//             None => false,
//         }
//     }
// }
//
// impl<'a> PartialEq<&'a Path> for Text {
//     fn eq(&self, other: &&'a Path) -> bool {
//         match other.to_str() {
//             Some(s) => self.text == s,
//             None => false,
//         }
//     }
// }
//
// impl PartialEq<PathBuf> for Text {
//     fn eq(&self, other: &PathBuf) -> bool {
//         match other.to_str() {
//             Some(s) => self.text == s,
//             None => false,
//         }
//     }
// }
//
// impl<'a> PartialEq<PathBuf> for &'a Text {
//     fn eq(&self, other: &PathBuf) -> bool {
//         match other.to_str() {
//             Some(s) => self.text == s,
//             None => false,
//         }
//     }
// }

// impl<T> From<T> for Text
//     where T: Textable
// {
//     fn from(s: T) -> Self {
//         s.to_text()
//     }
// }

// impl From<Text> for String {
//     fn from(t: Text) -> Self {
//         t.text
//     }
// }
//
// impl<'a> From<&'a Text> for &'a Path {
//     fn from(t: &'a Text) -> Self {
//         Path::new(&t.text)
//     }
// }
