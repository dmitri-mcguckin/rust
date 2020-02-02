#[derive(Debug, Default, Clone)]
pub struct KWIndex<'a>(Vec<&'a str>);

impl<'a> KWIndex<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn extend_from_text(mut self, target: &'a str) -> Self {
        for word in target.split(
            &[
                ' ', '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+',
                '=', '[', ']', '{', '}', ';', ':', '\"', '<', '>', ',', '.', '/', '?', '\\', '|',
            ][..],
        ) {
            if word != "" && !word.contains('\'') {
                self.0.push(word);
            }
        }
        self
    }

    pub fn count_matches(&self, keyword: &str) -> usize {
        let mut count = 0;
        for i in &self.0 {
            if i == &keyword {
                count += 1;
            }
        }
        count
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
