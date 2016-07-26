extern crate easystring;

#[cfg(test)]
mod length {
    use easystring::{Text, Textable};

    #[test]
    fn zero() {
        assert_eq!(Text::new().len(), 0);
    }

    #[test]
    fn non_zero() {
        let text = Text::from("1234567890");
        assert_eq!(text.len(), 10);
    }
}

#[cfg(test)]
mod from {
    use std::path::{Path, PathBuf};
    use easystring::{Text, Textable};

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
    use easystring::{Text, Textable};

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
}

#[cfg(test)]
mod partial_equality_as_ref {
    use std::path::{Path, PathBuf};
    use easystring::{Text, Textable};

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
}
