use std::convert::From;
use std::fmt;
use std::ops::Deref;
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

// DOES NOT WORK
// impl<T> PartialEq<T> for Text where T: Textable + Clone {
//     fn eq(&self, other: &T) -> bool {
//         let other = Text::from(other.clone());
//         self.text == other.text
//     }
// }

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

// impl<'a, T> PartialEq<T> for Text
//     where Text: From<&'a T>,
//           T: 'a
// {
//     fn eq(&self, other: &T) -> bool {
//         let other = Text::from(other);
//         self.text == other.text
//     }
// }

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


//////////////////////

impl<T> From<T> for Text
    where T: Textable
{
    fn from(s: T) -> Self {
        s.to_text()
    }
}

// impl<'a> From<&'a str> for Text {
//     fn from(s: &'a str) -> Self {
//         Text { text: s.to_owned() }
//     }
// }


// impl<T> From<T> for Text
//     where T: Into<String>
// {
//     default fn from(s: T) -> Self {
//         Text { text: s.into() }
//     }
// }

// impl<T> From<&T> for Text
//     where String: From<&T>
// {
//     fn from(s: &T) -> Self {
//         Text { text: String::from(s) }
//     }
// }


// impl<'a, T> From<T> for Text
//     where String: From<&'a T>,
//         T: 'a
// {
//     fn from(ref s: T) -> Self {
//         Text { text: String::from(s) }
//     }
// }

// impl From<String> for Text {
//     fn from(s: String) -> Self {
//         Text { text: s }
//     }
// }

// impl<'a> From<&'a Path> for Text {
//     fn from(s: &'a Path) -> Self {
//         Text { text: s.to_string_lossy().into_owned() }
//     }
// }
//
// impl From<PathBuf> for Text {
//     fn from(s: PathBuf) -> Self {
//         Text { text: s.to_string_lossy().into_owned() }
//     }
// }

// impl From<Text> for String {
//     fn from(t: Text) -> Self {
//         t.text
//     }
// }
