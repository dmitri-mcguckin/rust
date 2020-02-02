mod tests {
    #[test]
    fn count_zero_matches() {
        let text = "";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(0, kw.count_matches("world"));
    }

    #[test]
    fn count_one_match() {
        let text = "Hello world.";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(1, kw.count_matches("world"));
    }

    #[test]
    fn count_two_matches() {
        let text = "Hello world. what a  world.";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(2, kw.count_matches("world"));
    }

    #[test]
    fn check_empty() {
        let text = "";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(true, kw.is_empty());
    }

    #[test]
    fn check_not_empty() {
        let text = "Hello world.";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(false, kw.is_empty());
    }

    #[test]
    fn check_len_zero() {
        let text = "";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(0, kw.len());
    }

    #[test]
    fn check_len_not_zero() {
        let text = "Hello world.";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(2, kw.len());
    }

    #[test]
    fn check_len_is_five() {
        let text = "It ain't over untïl it ain't, over.";
        let kw = kwindex::KWIndex::new();
        let kw = kw.extend_from_text(text);
        assert_eq!(5, kw.len());
    }
}
