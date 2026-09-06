//! Spanish light stemmer.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Spanish light stemmer — removes plural and gender suffixes.
#[derive(Clone, Debug, Default)]
pub struct SpanishLightStemFilter;

impl SpanishLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for SpanishLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }
        let stemmed = stem_spanish_light(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_spanish_light(word: &str) -> String {
    let mut result = String::from(word);

    if result.ends_with("eses") {
        result.truncate(result.len() - 4);
        result.push_str("és");
        return result;
    }
    if result.ends_with("ces") {
        result.truncate(result.len() - 3);
        result.push('z');
        return result;
    }
    if result.ends_with("os") || result.ends_with("as") || result.ends_with("es") {
        result.truncate(result.len() - 2);
        return result;
    }
    if result.ends_with('o') || result.ends_with('a') || result.ends_with('e') {
        result.pop();
        return result;
    }
    if result.ends_with('s') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plural() {
        let f = SpanishLightStemFilter::new();
        let mut token = Token::new("gatos", 0, 5, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "gat");
    }

    #[test]
    fn test_ces() {
        let f = SpanishLightStemFilter::new();
        let mut token = Token::new("luces", 0, 5, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "luz");
    }
}
