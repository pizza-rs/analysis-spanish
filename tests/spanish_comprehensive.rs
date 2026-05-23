//! Comprehensive tests for pizza-analysis-spanish.

use pizza_analysis_spanish::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// SpanishLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stem_construction() {
    let _f = SpanishLightStemFilter::new();
}

#[test]
fn stem_plural_s() {
    let f = SpanishLightStemFilter::new();
    // "gatos" (cats) → stem
    let mut token = make_token("gatos");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_ne!(token.term.as_ref(), "gatos");
}

#[test]
fn stem_plural_es() {
    let f = SpanishLightStemFilter::new();
    // "flores" (flowers) → stem
    let mut token = make_token("flores");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_feminine_a() {
    let f = SpanishLightStemFilter::new();
    // "bonita" (pretty, f.) → stem
    let mut token = make_token("bonita");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_verb_infinitive() {
    let f = SpanishLightStemFilter::new();
    let mut token = make_token("corriendo");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_augmentative() {
    let f = SpanishLightStemFilter::new();
    let mut token = make_token("casona");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_diminutive() {
    let f = SpanishLightStemFilter::new();
    // "gatito" (little cat) → stem
    let mut token = make_token("gatito");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_adjective_plural() {
    let f = SpanishLightStemFilter::new();
    // "grandes" (big, pl.) → stem
    let mut token = make_token("grandes");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_short_word() {
    let f = SpanishLightStemFilter::new();
    let mut token = make_token("el");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_empty_string() {
    let f = SpanishLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn stem_single_char() {
    let f = SpanishLightStemFilter::new();
    let mut token = make_token("a");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// SpanishStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = SpanishStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = SpanishStopFilter::new();
    let stop_words = ["el", "la", "de", "en", "y", "que", "del", "los", "las", "un", "es", "no", "por"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = SpanishStopFilter::new();
    let content_words = ["casa", "libro", "escuela", "ciudad"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = SpanishStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("spanish_light_stem").is_some());
    assert!(factory.get_token_filter("spanish_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("spanish").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("spanish").unwrap();
    let mut input = String::from("La casa es grande y bonita");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("spanish").unwrap();
    let mut input = String::from("el gato en la casa");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"el"));
    assert!(!terms.contains(&"en"));
    assert!(!terms.contains(&"la"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("spanish").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_single_word() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("spanish").unwrap();
    let mut input = String::from("Madrid");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_ascii_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("spanish").unwrap();
    let mut input = String::from("hello world");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
