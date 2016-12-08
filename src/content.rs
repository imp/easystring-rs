use std::borrow::Cow;
use std::fmt;
use std::ffi::{CStr, CString, OsStr, OsString};
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
        unimplemented!()
    }
}

impl<T> PartialEq<T> for Content
    where String: Into<T>
{
    default fn eq(&self, other: &T) -> bool {
        unimplemented!()
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

impl<'a> ToContent<&'a str> for &'a str  {
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

impl<'a> ToContent<Cow<'a, str>> for Cow<'a, str>  {
    fn to_content(self) -> Content {
        Content::String(self.into())
    }
}

impl<'a> ToContent<Cow<'a, OsStr>> for Cow<'a, OsStr>  {
    fn to_content(self) -> Content {
        Content::OsString(self.into_owned())
    }
}

impl<'a> ToContent<Cow<'a, CStr>> for Cow<'a, CStr>  {
    fn to_content(self) -> Content {
        Content::CString(self.into_owned())
    }
}

impl<'a> ToContent<Cow<'a, Path>> for Cow<'a, Path>  {
    fn to_content(self) -> Content {
        Content::PathBuf(self.into_owned())
    }
}
