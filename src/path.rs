//! SHACL property paths.

use std::fmt;

use iri_rfc3987::Iri;

use crate::local_name;

/// A SHACL property path represented as an AST.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ShaclPath<Named = Iri<String>> {
    /// A single predicate IRI.
    Predicate(Named),
    /// A sequence of paths (`p1 / p2 / ...`).
    Sequence(Vec<ShaclPath<Named>>),
    /// An alternative of paths (`p1 | p2 | ...`).
    Alternative(Vec<ShaclPath<Named>>),
    /// The inverse of a path (`^p`).
    Inverse(Box<ShaclPath<Named>>),
    /// Zero or more repetitions (`p*`).
    ZeroOrMore(Box<ShaclPath<Named>>),
    /// One or more repetitions (`p+`).
    OneOrMore(Box<ShaclPath<Named>>),
    /// Zero or one occurrence (`p?`).
    ZeroOrOne(Box<ShaclPath<Named>>),
}

impl<Named: AsRef<str>> fmt::Display for ShaclPath<Named> {
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

fn write_paths<Named: AsRef<str>>(
    formatter: &mut fmt::Formatter<'_>,
    paths: &[ShaclPath<Named>],
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
