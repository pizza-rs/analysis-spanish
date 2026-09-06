//! Spanish stop words (from Lucene/Snowball project).

use alloc::borrow::Cow;
use alloc::vec::Vec;
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

/// Default Spanish stop words sourced from Apache Lucene.
static DEFAULT_STOP_WORDS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let words: &[&str] = &[
        "a",
        "al",
        "algo",
        "algunas",
        "algunos",
        "ante",
        "antes",
        "como",
        "con",
        "contra",
        "cual",
        "cuando",
        "de",
        "del",
        "desde",
        "donde",
        "durante",
        "e",
        "el",
        "ella",
        "ellas",
        "ellos",
        "en",
        "entre",
        "era",
        "erais",
        "eran",
        "eras",
        "eres",
        "es",
        "esa",
        "esas",
        "ese",
        "eso",
        "esos",
        "esta",
        "estaba",
        "estabais",
        "estaban",
        "estabas",
        "estad",
        "estada",
        "estadas",
        "estado",
        "estados",
        "estamos",
        "estando",
        "estar",
        "estaremos",
        "estará",
        "estarán",
        "estarás",
        "estaré",
        "estaréis",
        "estaría",
        "estaríais",
        "estaríamos",
        "estarían",
        "estarías",
        "estas",
        "este",
        "estemos",
        "esto",
        "estos",
        "estoy",
        "estuve",
        "estuviera",
        "estuvierais",
        "estuvieran",
        "estuvieras",
        "estuvieron",
        "estuviese",
        "estuvieseis",
        "estuviesen",
        "estuvieses",
        "estuvimos",
        "estuviste",
        "estuvisteis",
        "estuviéramos",
        "estuviésemos",
        "estuvo",
        "está",
        "estábamos",
        "estáis",
        "están",
        "estás",
        "esté",
        "estéis",
        "estén",
        "estés",
        "fue",
        "fuera",
        "fuerais",
        "fueran",
        "fueras",
        "fueron",
        "fuese",
        "fueseis",
        "fuesen",
        "fueses",
        "fui",
        "fuimos",
        "fuiste",
        "fuisteis",
        "fuéramos",
        "fuésemos",
        "ha",
        "habida",
        "habidas",
        "habido",
        "habidos",
        "habiendo",
        "habremos",
        "habrá",
        "habrán",
        "habrás",
        "habré",
        "habréis",
        "habría",
        "habríais",
        "habríamos",
        "habrían",
        "habrías",
        "habéis",
        "había",
        "habíais",
        "habíamos",
        "habían",
        "habías",
        "han",
        "has",
        "hasta",
        "hay",
        "haya",
        "hayamos",
        "hayan",
        "hayas",
        "hayáis",
        "he",
        "hemos",
        "hube",
        "hubiera",
        "hubierais",
        "hubieran",
        "hubieras",
        "hubieron",
        "hubiese",
        "hubieseis",
        "hubiesen",
        "hubieses",
        "hubimos",
        "hubiste",
        "hubisteis",
        "hubiéramos",
        "hubiésemos",
        "hubo",
        "la",
        "las",
        "le",
        "les",
        "lo",
        "los",
        "me",
        "mi",
        "mis",
        "mucho",
        "muchos",
        "muy",
        "más",
        "mí",
        "mía",
        "mías",
        "mío",
        "míos",
        "nada",
        "ni",
        "no",
        "nos",
        "nosotras",
        "nosotros",
        "nuestra",
        "nuestras",
        "nuestro",
        "nuestros",
        "o",
        "os",
        "otra",
        "otras",
        "otro",
        "otros",
        "para",
        "pero",
        "poco",
        "por",
        "porque",
        "que",
        "quien",
        "quienes",
        "qué",
        "se",
        "sea",
        "seamos",
        "sean",
        "seas",
        "seremos",
        "será",
        "serán",
        "serás",
        "seré",
        "seréis",
        "sería",
        "seríais",
        "seríamos",
        "serían",
        "serías",
        "seáis",
        "sido",
        "siendo",
        "sin",
        "sobre",
        "sois",
        "somos",
        "son",
        "soy",
        "su",
        "sus",
        "suya",
        "suyas",
        "suyo",
        "suyos",
        "sí",
        "también",
        "tanto",
        "te",
        "tendremos",
        "tendrá",
        "tendrán",
        "tendrás",
        "tendré",
        "tendréis",
        "tendría",
        "tendríais",
        "tendríamos",
        "tendrían",
        "tendrías",
        "tened",
        "tenemos",
        "tenga",
        "tengamos",
        "tengan",
        "tengas",
        "tengo",
        "tengáis",
        "tenida",
        "tenidas",
        "tenido",
        "tenidos",
        "teniendo",
        "tenéis",
        "tenía",
        "teníais",
        "teníamos",
        "tenían",
        "tenías",
        "ti",
        "tiene",
        "tienen",
        "tienes",
        "todo",
        "todos",
        "tu",
        "tus",
        "tuve",
        "tuviera",
        "tuvierais",
        "tuvieran",
        "tuvieras",
        "tuvieron",
        "tuviese",
        "tuvieseis",
        "tuviesen",
        "tuvieses",
        "tuvimos",
        "tuviste",
        "tuvisteis",
        "tuviéramos",
        "tuviésemos",
        "tuvo",
        "tuya",
        "tuyas",
        "tuyo",
        "tuyos",
        "tú",
        "un",
        "una",
        "uno",
        "unos",
        "vosotras",
        "vosotros",
        "vuestra",
        "vuestras",
        "vuestro",
        "vuestros",
        "y",
        "ya",
        "yo",
        "él",
        "éramos",
    ];
    words.iter().copied().collect()
});

/// Removes Spanish stop words from the token stream.
#[derive(Clone, Debug)]
pub struct SpanishStopFilter {
    stop_words: HashSet<String>,
}

impl Default for SpanishStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl SpanishStopFilter {
    pub fn new() -> Self {
        Self {
            stop_words: DEFAULT_STOP_WORDS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn with_words(words: &[&str]) -> Self {
        Self {
            stop_words: words.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl TokenFilter for SpanishStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.stop_words.contains(term) {
            return (true, None);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stop_word_count() {
        assert!(DEFAULT_STOP_WORDS.len() >= 308);
    }

    #[test]
    fn test_filters_stop_word() {
        let f = SpanishStopFilter::new();
        let word = DEFAULT_STOP_WORDS.iter().next().unwrap();
        let mut token = Token::new(word, 0, word.len() as u32, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }

    #[test]
    fn test_passes_non_stop_word() {
        let f = SpanishStopFilter::new();
        let mut token = Token::new("xyzzy_not_a_stop_word", 0, 21, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted);
    }

    #[test]
    fn test_custom_words() {
        let f = SpanishStopFilter::with_words(&["custom", "words"]);
        let mut token = Token::new("custom", 0, 6, 0);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted);
    }
}
