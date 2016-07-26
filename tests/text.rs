extern crate easystring;

#[cfg(test)]
mod length {
    use easystring::Text;

    #[test]
    fn zero() {
        assert_eq!(Text::new().len(), 0);
    }

    #[test]
    fn non_zero() {
        let text = Text::from("1234567890");
        assert_eq!(text.len(), 10);
    }

    #[test]
    fn empty() {
        assert!(Text::new().is_empty());
    }

    #[test]
    fn not_empty() {
        let text = Text::from("1234567890");
        assert!(!text.is_empty());
    }
}

#[cfg(test)]
mod from {
    use std::path::{Path, PathBuf};
    use easystring::Text;

    const STRING: &'static str = "EaSyStRiNg";
    #[test]
    fn str() {
        let text = Text::from(STRING);
        assert_eq!(&text, STRING);
    }

    #[test]
    fn string() {
        let string = String::from(STRING);
        let text = Text::from(string);
        assert_eq!(&text, STRING);
    }

    #[test]
    fn path() {
        let path = Path::new(STRING);
        let text = Text::from(path);
        assert_eq!(&text, STRING)
    }

    #[test]
    fn pathbuf() {
        let path = PathBuf::from(STRING);
        let text = Text::from(path);
        assert_eq!(&text, STRING)
    }
}

#[cfg(test)]
mod partial_equality_direct {
    use std::path::{Path, PathBuf};
    use easystring::Text;

    const STRING: &'static str = "EaSyStRiNg";

    #[test]
    fn str() {
        let text = Text::from(STRING);
        assert_eq!(text, STRING);
    }

    #[test]
    fn string() {
        let text = Text::from(STRING);
        let string = String::from(STRING);
        assert_eq!(text, string);
    }

    #[test]
    fn path() {
        let path = Path::new(STRING);
        let text = Text::from(STRING);
        assert_eq!(text, path);
    }

    #[test]
    fn pathbuf() {
        let pathbuf = PathBuf::from(STRING);
        let text = Text::from(STRING);
        assert_eq!(text, pathbuf);
    }
}

#[cfg(test)]
mod partial_equality_as_ref {
    use std::path::{Path, PathBuf};
    use easystring::Text;

    const STRING: &'static str = "EaSyStRiNg";

    #[test]
    fn str() {
        let text = Text::from(STRING);
        assert_eq!(&text, STRING);
    }

    #[test]
    fn string() {
        let text = Text::from(STRING);
        let string = String::from(STRING);
        assert_eq!(&text, string);
    }

    #[test]
    fn path() {
        let path = Path::new(STRING);
        let text = Text::from(STRING);
        assert_eq!(&text, path);
    }

    #[test]
    fn pathbuf() {
        let pathbuf = PathBuf::from(STRING);
        let text = Text::from(STRING);
        assert_eq!(&text, pathbuf);
    }
}

#[cfg(test)]
mod ops {
    use std::path::{Path, PathBuf};
    use easystring::Text;

    #[test]
    fn add_text() {
        let t1 = Text::from("123");
        let t2 = Text::from("456");
        let text = t1 + t2;
        assert_eq!(text, "123456");
    }

    #[test]
    fn add_string() {
        let t1 = Text::from("123");
        let t2 = String::from("456");
        let text = t1 + t2;
        assert_eq!(text, "123456");
    }

    #[test]
    fn add_path() {
        let t1 = Text::from("123");
        let t2 = Path::new("456");
        let text = t1 + t2;
        assert_eq!(text, "123456");
    }

    #[test]
    fn add_pathbuf() {
        let t1 = Text::from("123");
        let t2 = PathBuf::from("456");
        let text = t1 + t2;
        assert_eq!(text, "123456");
    }
}
