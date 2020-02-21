use super::DecodingTool;

pub struct Rot13;

impl DecodingTool for Rot13 {
    fn decode(&self, input: &str) -> Option<String> {
        self.debugok();
        Some(rot13(&input))
    }

    fn name(&self) -> String {
        String::from("Rot 13")
    }
}

/// code stolen on https://github.com/marekventur/rust-rot13/blob/master/src/lib.rs
///
/// Encodes a String as ROT13.
/// Maintains lower/upper case, ignores all non [A-Za-z] characters.
///
/// # Examples
///
/// ```
/// use rot13::rot13;
///
/// assert_eq!("Uryyb Jbeyq!", rot13("Hello World!"));
/// assert_eq!("Test", rot13(&rot13("Test")));
/// ```

fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => ((c as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod test {

    use super::rot13;

    #[test]
    fn test_simple_case() {
        assert_eq!("nopq", rot13("abcd"));
    }

    #[test]
    fn test_with_non_latin_characters() {
        assert_eq!("Jöegre", rot13("Wörter"));
    }

    #[test]
    fn test_with_non_ascii_characters() {
        assert_eq!("n😋o", rot13("a😋b"));
    }

    #[test]
    fn test_can_be_reversed_by_applying_twice() {
        assert_eq!("Hello World!", rot13(&rot13("Hello World!")));
    }
}
