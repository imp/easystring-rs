use std::borrow::Cow;
use std::fmt;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::ops::{Index, Range, RangeFull};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum Content {
    String(String),
    OsString(OsString),
    CString(CString),
    PathBuf(PathBuf),
}

impl Content {
    pub fn len(&self) -> usize {
        match *self {
            Content::String(ref s) => s.len(),
            Content::OsString(ref s) => s.len(),
            Content::CString(ref s) => s.as_bytes().len(),
            Content::PathBuf(ref p) => p.as_os_str().len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            Content::String(ref s) => s.is_empty(),
            Content::OsString(ref s) => s.is_empty(),
            Content::CString(ref s) => s.as_bytes().is_empty(),
            Content::PathBuf(ref p) => p.as_os_str().is_empty(),
        }
    }

    pub fn as_os_str(&self) -> &OsStr {
        match *self {
            Content::String(ref s) => s.as_ref(),
            Content::OsString(ref s) => s,
            Content::CString(ref s) => OsStr::from_bytes(s.as_bytes()),
            Content::PathBuf(ref p) => p.as_os_str(),
        }
    }
}

impl Index<RangeFull> for Content {
    type Output = OsStr;
    fn index(&self, index: RangeFull) -> &OsStr {
        match *self {
            Content::String(ref s) => s.as_ref(),
            Content::OsString(ref s) => s.index(index),
            Content::CString(ref s) => OsStr::from_bytes(s.as_bytes().index(index)),
            Content::PathBuf(ref p) => p.as_os_str(),
        }
    }
}

impl Index<Range<usize>> for Content {
    type Output = OsStr;
    fn index(&self, index: Range<usize>) -> &OsStr {
        OsStr::from_bytes(self[..].as_bytes().index(index))
    }
}

impl<T> From<T> for Content
    where T: ToContent<T>
{
    fn from(s: T) -> Self {
        s.to_content()
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{}", self))
    }
}

impl PartialEq for Content {
    fn eq(&self, other: &Content) -> bool {
        self.as_os_str() == other.as_os_str()
    }
}

macro_rules! impl_eq {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool { PartialEq::eq(&self[..], &other[..]) }
            #[inline]
            fn ne(&self, other: &$rhs) -> bool { PartialEq::ne(&self[..], &other[..]) }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool { PartialEq::eq(&self[..], &other[..]) }
            #[inline]
            fn ne(&self, other: &$lhs) -> bool { PartialEq::ne(&self[..], &other[..]) }
        }

    }
}

impl PartialEq<String> for Content {
    fn eq(&self, other: &String) -> bool {
        self.as_os_str().as_bytes() == other.as_bytes()
    }
}

impl<'a> PartialEq<&'a str> for Content {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_os_str().as_bytes() == other.as_bytes()
    }
}

impl<'a> PartialEq<&'a CStr> for Content {
    fn eq(&self, other: &&'a CStr) -> bool {
        self.as_os_str().as_bytes() == other.to_bytes()
    }
}

// impl<T: AsRef<CStr>> PartialEq<T> for Content {
//     fn eq(&self, other: &T) -> bool {
//         self.as_os_str() == OsStr::from_bytes(other.as_ref().as_bytes())
//     }
// }

impl PartialEq<CString> for Content {
    fn eq(&self, other: &CString) -> bool {
        self.as_os_str().as_bytes() == other.as_bytes()
    }
}

impl<T: AsRef<OsStr>> PartialEq<T> for Content {
    default fn eq(&self, other: &T) -> bool {
        self.as_os_str() == other.as_ref()
    }
}


pub trait ToContent<T> {
    fn to_content(self) -> Content;
}

impl ToContent<String> for String {
    fn to_content(self) -> Content {
        Content::String(self)
    }
}

impl<'a> ToContent<&'a str> for &'a str {
    fn to_content(self) -> Content {
        Content::String(self.into())
    }
}

impl ToContent<OsString> for OsString {
    fn to_content(self) -> Content {
        Content::OsString(self)
    }
}

impl ToContent<CString> for CString {
    fn to_content(self) -> Content {
        Content::CString(self)
    }
}

impl ToContent<PathBuf> for PathBuf {
    fn to_content(self) -> Content {
        Content::PathBuf(self)
    }
}

impl<'a> ToContent<&'a Path> for &'a Path {
    fn to_content(self) -> Content {
        Content::PathBuf(self.into())
    }
}

impl<'a> ToContent<Cow<'a, str>> for Cow<'a, str> {
    fn to_content(self) -> Content {
        Content::String(self.into())
    }
}

impl<'a> ToContent<Cow<'a, OsStr>> for Cow<'a, OsStr> {
    fn to_content(self) -> Content {
        Content::OsString(self.into_owned())
    }
}

impl<'a> ToContent<Cow<'a, CStr>> for Cow<'a, CStr> {
    fn to_content(self) -> Content {
        Content::CString(self.into_owned())
    }
}

impl<'a> ToContent<Cow<'a, Path>> for Cow<'a, Path> {
    fn to_content(self) -> Content {
        Content::PathBuf(self.into_owned())
    }
}
