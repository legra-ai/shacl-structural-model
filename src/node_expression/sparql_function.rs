//! The SPARQL functions usable as SHACL 1.2 list parameter functions.
//!
//! SHACL 1.2 Node Expressions, Working Draft 16 November 2026, section 5,
//! declares each function as `sparql:<NAME>` in the `http://www.w3.org/ns/sparql#`
//! namespace. This is the draft's closed table; arity, argument types and
//! evaluation follow SPARQL 1.2 and are consumer-owned.

use crate::vocabulary;

/// Declare the closed function table once, with each draft local name.
macro_rules! sparql_functions {
    ($($variant:ident => $local:literal,)+) => {
        /// One `sparql:<NAME>` function from the draft's section 5 table.
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord,
            serde::Serialize, serde::Deserialize,
        )]
        pub enum SparqlFunction {
            $(
                #[doc = concat!("`sparql:", $local, "`.")]
                $variant,
            )+
        }

        impl SparqlFunction {
            /// Every function of the draft table, in table order.
            pub const ALL: &[Self] = &[$(Self::$variant,)+];

            /// The draft's local name, such as `encode-for-uri`.
            #[must_use]
            pub const fn local_name(self) -> &'static str {
                match self {
                    $(Self::$variant => $local,)+
                }
            }

            /// The function's full list parameter property IRI.
            #[must_use]
            pub const fn as_iri(self) -> &'static str {
                match self {
                    $(Self::$variant => concat!("http://www.w3.org/ns/sparql#", $local),)+
                }
            }
        }
    };
}

sparql_functions! {
    Abs => "abs",
    Bnode => "bnode",
    Bound => "bound",
    Ceil => "ceil",
    Coalesce => "coalesce",
    Concat => "concat",
    Contains => "contains",
    Datatype => "datatype",
    Day => "day",
    Divide => "divide",
    EncodeForUri => "encode-for-uri",
    Equals => "equals",
    Floor => "floor",
    GreaterThan => "greater-than",
    GreaterThanOrEqual => "greater-than-or-equal",
    HasLang => "hasLang",
    HasLangdir => "hasLangdir",
    Hours => "hours",
    If => "if",
    Iri => "iri",
    IsBlank => "isBlank",
    IsIri => "isIRI",
    IsLiteral => "isLiteral",
    IsNumeric => "isNumeric",
    IsTriple => "isTriple",
    IsUri => "isURI",
    Lang => "lang",
    LangMatches => "langMatches",
    Langdir => "langdir",
    Lcase => "lcase",
    LessThan => "less-than",
    LessThanOrEqual => "less-than-or-equal",
    LogicalAnd => "logical-and",
    LogicalNot => "logical-not",
    LogicalOr => "logical-or",
    Md5 => "md5",
    Minutes => "minutes",
    Month => "month",
    Multiply => "multiply",
    NotEquals => "not-equals",
    Now => "now",
    Object => "object",
    Plus => "plus",
    Predicate => "predicate",
    Rand => "rand",
    Regex => "regex",
    Replace => "replace",
    Round => "round",
    SameTerm => "sameTerm",
    SameValue => "sameValue",
    Seconds => "seconds",
    Sha1 => "sha1",
    Sha256 => "sha256",
    Sha384 => "sha384",
    Sha512 => "sha512",
    Str => "str",
    Strafter => "strafter",
    Strbefore => "strbefore",
    Strdt => "strdt",
    Strends => "strends",
    Strlang => "strlang",
    Strlangdir => "strlangdir",
    Strlen => "strlen",
    Strstarts => "strstarts",
    Struuid => "struuid",
    Subject => "subject",
    Substr => "substr",
    Subtract => "subtract",
    Timezone => "timezone",
    Triple => "triple",
    Tz => "tz",
    Ucase => "ucase",
    UnaryMinus => "unary-minus",
    UnaryPlus => "unary-plus",
    Uri => "uri",
    Uuid => "uuid",
    Year => "year",
}

impl SparqlFunction {
    /// Look up a function by its full `sparql:` IRI.
    #[must_use]
    pub fn from_iri(iri: &str) -> Option<Self> {
        let local = iri.strip_prefix(vocabulary::SPARQL_NAMESPACE)?;
        Self::ALL
            .iter()
            .copied()
            .find(|function| function.local_name() == local)
    }

    /// Whether the draft lets this function receive unbound arguments.
    ///
    /// Section 5 names `sparql:bound`, `sparql:coalesce`, `sparql:if`,
    /// `sparql:logical-and` and `sparql:logical-or`. For every other function an
    /// argument without output nodes makes the expression produce no output nodes.
    #[must_use]
    pub const fn handles_unbound(self) -> bool {
        matches!(
            self,
            Self::Bound | Self::Coalesce | Self::If | Self::LogicalAnd | Self::LogicalOr
        )
    }
}
