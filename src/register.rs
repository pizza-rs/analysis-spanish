//! Register Spanish analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::{
    Analyzer, AnalysisFactory, LowercaseNormalizer, Normalizer, StandardTokenizer, TokenFilter,
    Tokenizer,
};

use crate::{SpanishLightStemFilter, SpanishStopFilter};

/// Register Spanish token filters and the `"spanish"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("spanish_light_stem", Box::new(SpanishLightStemFilter::new()));
    factory.register_token_filter("spanish_stop", Box::new(SpanishStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(SpanishStopFilter::new()),
        Box::new(SpanishLightStemFilter::new()),
    ];
    factory.register_analyzer("spanish", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("spanish_light_stem").is_some());
        assert!(factory.get_token_filter("spanish_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("spanish").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("spanish").unwrap();
        let mut input = String::from("El gato está en la casa");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        assert!(!tokens.iter().any(|t| t.term == "el"));
        assert!(!tokens.iter().any(|t| t.term == "en"));
        assert!(!tokens.iter().any(|t| t.term == "la"));
        assert!(tokens.len() >= 2);
    }
}
