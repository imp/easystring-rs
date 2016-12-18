extern crate easystring;

#[cfg(test)]
mod content {
    mod index {
        use std::ffi::{CStr, CString, OsStr, OsString};
        use std::path::{Path, PathBuf};
        use easystring::{Content, ToContent};

        #[test]
        fn slice() {
            let content = Content::from("abcde");
            assert_eq!(content, "abcde");
        }
    }

    mod compare {
        use std::ffi::{CStr, CString, OsStr, OsString};
        use std::path::{Path, PathBuf};
        use easystring::{Content, ToContent};

        #[test]
        fn with_str() {
            let conent = Content::from("12345");
            assert_eq!(conent, "12345");
        }

        #[test]
        fn with_string() {
            let conent = Content::from("12345");
            let string = String::from("12345");
            assert_eq!(conent, string);
        }

        #[test]
        fn with_osstr() {
            let conent = Content::from("12345");
            let osstr = OsStr::new("12345");
            assert_eq!(conent, osstr);
        }

        #[test]
        fn with_osstring() {
            let content = Content::from("12345");
            let osstring = OsString::from("12345");
            assert_eq!(content, osstring);
        }

        #[test]
        fn with_cstr() {
            let content = Content::from("12345");
            let cstring = CString::new(b"12345".to_vec()).unwrap();
            let cstr = cstring.as_ref();
            assert_eq!(content, cstr);
        }

        #[test]
        fn with_cstring() {
            let content = Content::from("12345");
            let cstring = CString::new(b"12345".to_vec()).unwrap();
            assert_eq!(content, cstring);
        }

        #[test]
        fn with_path() {
            let conent = Content::from("12345");
            let path = Path::new("12345");
            assert_eq!(conent, path);
        }

        #[test]
        fn with_pathbuf() {
            let conent = Content::from("12345");
            let pathbuf = PathBuf::from("12345");
            assert_eq!(conent, pathbuf);
        }
    }
}
