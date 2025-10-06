use regex::Regex;

pub fn retrieve_capture(line: &str, pattern: &str) -> Option<String> {
    let reg = Regex::new(pattern).unwrap();

    if let Some(caps) = reg.captures(line) {
        return caps.get(1).map(|x| x.as_str().to_string());
    };

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expect_cap_to_have_some() {
        // Capture "Bar" between any chars
        let pattern = r"\w* (Bar) \w*";
        let line = "Foo Bar Foo";
        let cap = retrieve_capture(line, pattern);
        assert!(cap.is_some())
    }

    #[test]
    fn expect_cap_to_be_bar() {
        // Capture "Bar" between any chars
        let pattern = r"\w* (Bar) \w*";
        let line = "Foo Bar Foo";
        let cap = retrieve_capture(line, pattern).unwrap();
        assert_eq!(cap, "Bar")
    }

    #[test]
    fn expect_cap_to_have_none() {
        // Capture "Bar" between any chars
        let pattern = r"\w* (Bar) \w*";
        let line = "Foo Foo";
        let cap = retrieve_capture(line, pattern);
        assert!(cap.is_none())
    }

    #[test]
    fn expect_cap_to_have_none_when_line_empty() {
        // Capture "Bar" between any chars
        let pattern = r"\w* (Bar) \w*";
        let line = "";
        let cap = retrieve_capture(line, pattern);
        assert!(cap.is_none())
    }

    #[test]
    fn expect_to_return_the_first_capture() {
        // Capture any word containing "ello"
        let pattern = r"(\wello\w).*";
        let line = "Yellow Mellow";
        let cap = retrieve_capture(line, pattern).unwrap();
        assert_eq!(cap, "Yellow")
    }
}
