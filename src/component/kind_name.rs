//! SHACL constraint kind names.

use super::Component;

impl<Value, Count, Flags, Pattern, Named> Component<Value, Count, Flags, Pattern, Named> {
    /// Return the SHACL constraint kind name.
    #[must_use]
    pub const fn kind_name(&self) -> &'static str {
        match self {
            Self::Class(_) => "class",
            Self::Datatype(_) => "datatype",
            Self::NodeKind(_) => "nodeKind",
            Self::MinCount(_) => "minCount",
            Self::MaxCount(_) => "maxCount",
            Self::MinExclusive(_) => "minExclusive",
            Self::MinInclusive(_) => "minInclusive",
            Self::MaxExclusive(_) => "maxExclusive",
            Self::MaxInclusive(_) => "maxInclusive",
            Self::MinLength(_) => "minLength",
            Self::MaxLength(_) => "maxLength",
            Self::Pattern { .. } => "pattern",
            Self::LanguageIn(_) => "languageIn",
            Self::UniqueLang(_) => "uniqueLang",
            Self::Equals(_) => "equals",
            Self::Disjoint(_) => "disjoint",
            Self::LessThan(_) => "lessThan",
            Self::LessThanOrEquals(_) => "lessThanOrEquals",
            Self::And(_) => "and",
            Self::Or(_) => "or",
            Self::Not(_) => "not",
            Self::Xone(_) => "xone",
            Self::Node(_) => "node",
            Self::QualifiedValueShape { .. } => "qualifiedValueShape",
            Self::HasValue(_) => "hasValue",
            Self::In(_) => "in",
            Self::Closed { .. } => "closed",
            Self::Sparql(_) => "sparql",
            Self::Expression(_) => "expression",
            Self::Custom(_) => "custom",
            Self::SingleLine(_) => "singleLine",
            Self::MemberShape(_) => "memberShape",
            Self::MinListLength(_) => "minListLength",
            Self::MaxListLength(_) => "maxListLength",
            Self::UniqueMembers(_) => "uniqueMembers",
            Self::SubsetOf(_) => "subsetOf",
            Self::SomeValue(_) => "someValue",
            Self::ReifierShape { .. } => "reifierShape",
            Self::RootClass(_) => "rootClass",
            Self::UniqueValuesFor(_) => "uniqueValuesFor",
        }
    }
}
