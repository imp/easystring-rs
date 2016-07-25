extern crate text;

#[cfg(test)]
mod tests {
    use text::Text;

    // #[test]
    // fn typename_as_constructor() {
    //     let t = Text(String::from("XYZ"));
    //     println!("{:?}", t);
    //     assert!(false);
    // }

    #[test]
    fn from_str() {
        let text = Text::from("ABCDEFG");
        assert_eq!(&text, "ABCDEFG");
    }

    #[test]
    fn from_string() {
        let string = String::from("STRING");
        let text = Text::from(string);
        assert_eq!(&text, "STRING");
    }
}
