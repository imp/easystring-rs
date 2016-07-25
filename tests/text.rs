extern crate easystring;

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
