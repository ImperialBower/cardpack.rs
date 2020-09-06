use std::fmt;

/// Karten Anzug Name (Card Suit Name) - Single field struct representing the name of a card suit.
///
#[derive(Clone, Debug, PartialEq)]
pub struct AnzugName(String);

impl AnzugName {

    // Accepts String or &str
    // https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html#another-way-to-write-personnew
    fn new<S>(name: S) -> AnzugName where S: Into<String> {
        AnzugName(name.into())
    }

    fn as_str(&self) -> &str {
        return self.0.as_str()
    }
}

impl fmt::Display for AnzugName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0,
        )
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod suite_name_tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!("Anzug: foo", format!("Anzug: {}", AnzugName::new("foo")));
    }

    #[test]
    fn as_str() {
        assert_eq!(AnzugName::new("bar").as_str(), "bar");
    }

    #[test]
    fn to_string() {
        assert_eq!(AnzugName("foo".to_string()).to_string(), "foo".to_string());
    }

    #[test]
    fn new() {
        let from_string = "from".to_string();

        assert_eq!(AnzugName("from".to_string()), AnzugName::new(from_string));
        assert_eq!(AnzugName("from".to_string()), AnzugName::new("from"));
    }
}