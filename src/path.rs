//! SHACL property paths.

use std::fmt;

use iri_rfc3987::Iri;

use crate::local_name;

/// A SHACL property path represented as an AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ShaclPath {
    /// A single predicate IRI.
    Predicate(Iri<String>),
    /// A sequence of paths (`p1 / p2 / ...`).
    Sequence(Vec<ShaclPath>),
    /// An alternative of paths (`p1 | p2 | ...`).
    Alternative(Vec<ShaclPath>),
    /// The inverse of a path (`^p`).
    Inverse(Box<ShaclPath>),
    /// Zero or more repetitions (`p*`).
    ZeroOrMore(Box<ShaclPath>),
    /// One or more repetitions (`p+`).
    OneOrMore(Box<ShaclPath>),
    /// Zero or one occurrence (`p?`).
    ZeroOrOne(Box<ShaclPath>),
}

impl fmt::Display for ShaclPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Predicate(iri) => write!(formatter, "{}", local_name(iri)),
            Self::Sequence(paths) => write_paths(formatter, paths, " / "),
            Self::Alternative(paths) => write_paths(formatter, paths, " | "),
            Self::Inverse(inner) => write!(formatter, "^{inner}"),
            Self::ZeroOrMore(inner) => write!(formatter, "{inner}*"),
            Self::OneOrMore(inner) => write!(formatter, "{inner}+"),
            Self::ZeroOrOne(inner) => write!(formatter, "{inner}?"),
        }
    }
}

fn write_paths(
    formatter: &mut fmt::Formatter<'_>,
    paths: &[ShaclPath],
    separator: &str,
) -> fmt::Result {
    for (index, path) in paths.iter().enumerate() {
        if index > 0 {
            write!(formatter, "{separator}")?;
        }
        write!(formatter, "{path}")?;
    }
    Ok(())
}
