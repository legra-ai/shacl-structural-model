//! Census: every `sh:` term of the vendored W3C vocabularies is accounted for.
//!
//! Each term is either modelled, on the commented not-yet-modelled list,
//! superseded by its SHACL 1.2 replacement, or a SHACL-JS term. A newly vendored vocabulary term fails this test until it is
//! classified, and a classification of a term the vocabularies no longer
//! declare fails too.
//!
//! Term extraction is a deliberately tiny line-oriented scan rather than a
//! Turtle parser dependency: both vocabulary files declare every term as a
//! subject that starts a line (`sh:minCount` at column 0). Only those subject
//! lines are read; objects and nested `sh:` uses are not terms of their own.

#[path = "vocabulary_census/modelled.rs"]
mod modelled;
#[path = "vocabulary_census/not_yet_modelled.rs"]
mod not_yet_modelled;

use std::collections::BTreeSet;
use std::path::PathBuf;

use modelled::MODELLED;
use not_yet_modelled::{NOT_YET_MODELLED, SHACL_JS, SUPERSEDED};

const VOCABULARIES: [&str; 2] = ["shacl.ttl", "shacl12.ttl"];
const COMPONENT_SUFFIX: &str = "ConstraintComponent";

/// The local names of every subject `sh:` term in one vendored vocabulary.
fn declared_terms(file: &str) -> BTreeSet<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/shacl-vocabulary")
        .join(file);
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| line.strip_prefix("sh:"))
        .filter_map(|rest| rest.split_whitespace().next())
        .filter(|local| !local.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Classify `sh:XConstraintComponent[-param]` by its parameter term.
fn classified_as(term: &str) -> String {
    if let Some((component, parameter)) = term.split_once('-')
        && component.ends_with(COMPONENT_SUFFIX)
    {
        return parameter.to_owned();
    }
    match term.strip_suffix(COMPONENT_SUFFIX) {
        Some(prefix) if prefix.starts_with(|first: char| first.is_ascii_uppercase()) => {
            let mut chars = prefix.chars();
            chars.next().map_or_else(String::new, |first| {
                first.to_ascii_lowercase().to_string() + chars.as_str()
            })
        }
        _ => term.to_owned(),
    }
}

fn classifications(term: &str) -> usize {
    MODELLED.iter().filter(|(name, _)| *name == term).count()
        + NOT_YET_MODELLED
            .iter()
            .filter(|(name, _)| *name == term)
            .count()
        + SUPERSEDED.iter().filter(|(name, _)| *name == term).count()
        + SHACL_JS.iter().filter(|name| **name == term).count()
}

fn all_terms() -> BTreeSet<String> {
    VOCABULARIES
        .iter()
        .flat_map(|file| declared_terms(file))
        .collect()
}

#[test]
fn vendored_vocabularies_declare_their_terms_at_line_start() {
    for file in VOCABULARIES {
        let terms = declared_terms(file);
        assert!(terms.contains("minCount"), "{file} lost sh:minCount");
        assert!(
            terms.len() > 200,
            "{file} declares only {} terms",
            terms.len()
        );
    }
    assert!(declared_terms("shacl12.ttl").contains("singleLine"));
}

#[test]
fn every_vocabulary_term_is_classified_exactly_once() {
    let unclassified: Vec<_> = all_terms()
        .into_iter()
        .filter(|term| {
            let target = classified_as(term);
            let direct = classifications(term);
            let count = if direct == 0 {
                classifications(&target)
            } else {
                direct
            };
            count != 1
        })
        .collect();
    assert!(
        unclassified.is_empty(),
        "classify each of: {unclassified:?}"
    );
}

#[test]
fn every_classification_names_a_declared_term() {
    let terms = all_terms();
    let stale: Vec<_> = MODELLED
        .iter()
        .map(|(term, _)| *term)
        .chain(NOT_YET_MODELLED.iter().map(|(term, _)| *term))
        .chain(SUPERSEDED.iter().map(|(term, _)| *term))
        .chain(SHACL_JS.iter().copied())
        .filter(|term| !terms.contains(*term))
        .collect();
    assert!(stale.is_empty(), "no longer declared: {stale:?}");
}
