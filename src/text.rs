use std::convert::From;
use std::fmt;
use std::ops::{Add, AddAssign, Deref};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use super::content::{Content, ToContent};

#[derive(Debug, Default)]
pub struct Text {
    text: Vec<Content>,
}

impl<T> From<T> for Text
    where T: ToContent<T>
{
    fn from(s: T) -> Self {
        Text { text: vec![Content::from(s)] }
    }
}

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
    type Target = OsStr;
    fn deref(&self) -> &OsStr {
        if self.is_empty() {
            OsStr::new("")
        } else {
            match self.text[0] {
                Content::String(ref s) => s.as_ref(),
                Content::OsString(ref s) => s,
                Content::CString(ref s) => OsStr::from_bytes(s.as_bytes()),
                Content::PathBuf(ref p) => p.as_ref(),
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

impl<T> Add<T> for Text
    where T: ToContent<T>
{
    type Output = Text;
    fn add(self, rhs: T) -> Self::Output {
        let mut text = self.text.clone();
        text.push(Content::from(rhs));
        Text { text: text }
    }
}

// AddAssign implementation

impl AddAssign for Text {
    fn add_assign(&mut self, other: Self) {
        self.text.extend_from_slice(&other.text)
    }
}

// PartialEq implementation

// impl<T: Borrow<OsStr> + ?Sized> PartialEq<T> for Text {
//     default fn eq(&self, other: &T) -> bool {
//         unimplemented!()
//     }
// }

// impl<T: Borrow<Path> + ?Sized> PartialEq<T> for Text {
//     default fn eq(&self, other: &T) -> bool {
//         unimplemented!()
//     }
// }

impl PartialEq<str> for Text {
    fn eq(&self, other: &str) -> bool {
        unimplemented!()
    }
}

impl<'a> PartialEq<&'a str> for Text {
    fn eq(&self, other: &&'a str) -> bool {
        unimplemented!()
    }
}

impl PartialEq<String> for Text {
    fn eq(&self, other: &String) -> bool {
        unimplemented!()
    }
}

impl<'a> PartialEq<String> for &'a Text {
    fn eq(&self, other: &String) -> bool {
        unimplemented!()
        // self.text == *other
    }
}

impl PartialEq<Path> for Text {
    fn eq(&self, other: &Path) -> bool {
        unimplemented!()
    }
}

impl<'a> PartialEq<&'a Path> for Text {
    fn eq(&self, other: &&'a Path) -> bool {
        unimplemented!()
    }
}

impl PartialEq<PathBuf> for Text {
    fn eq(&self, other: &PathBuf) -> bool {
        unimplemented!()
    }
}

impl<'a> PartialEq<PathBuf> for &'a Text {
    fn eq(&self, other: &PathBuf) -> bool {
        unimplemented!()
    }
}

// impl<T> From<T> for Text
//     where T: Textable
// {
//     fn from(s: T) -> Self {
//         s.to_text()
//     }
// }

impl From<Text> for String {
    fn from(t: Text) -> Self {
        unimplemented!()
    }
}

impl<'a> From<&'a Text> for &'a Path {
    fn from(t: &'a Text) -> Self {
        unimplemented!()
    }
}
