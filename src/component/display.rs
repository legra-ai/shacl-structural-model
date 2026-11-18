//! Human-readable component display values.

use std::fmt;

use super::Component;
use crate::LanguageTag;

impl<
    Value: fmt::Debug,
    Count: fmt::Debug,
    Flags: fmt::Debug,
    Pattern: fmt::Debug,
    Named: AsRef<str>,
> Component<Value, Count, Flags, Pattern, Named>
{
    /// Return a human-readable display value for this component.
    #[must_use]
    pub fn display_value(&self) -> String {
        match self {
            Self::Class(iri) | Self::Datatype(iri) => crate::local_name(iri),
            Self::NodeKind(kind) => format!("{kind:?}"),
            Self::MinCount(count)
            | Self::MaxCount(count)
            | Self::MinLength(count)
            | Self::MaxLength(count)
            | Self::MinListLength(count)
            | Self::MaxListLength(count) => format!("{count:?}"),
            Self::MinExclusive(value)
            | Self::MinInclusive(value)
            | Self::MaxExclusive(value)
            | Self::MaxInclusive(value)
            | Self::HasValue(value)
            | Self::Not(value)
            | Self::Node(value)
            | Self::MemberShape(value)
            | Self::SomeValue(value) => format!("{value:?}"),
            Self::Pattern { pattern, flags } => match flags {
                Some(flags) => format!("/{pattern:?}/{flags:?}"),
                None => format!("/{pattern:?}/"),
            },
            Self::LanguageIn(languages) => languages
                .iter()
                .map(LanguageTag::as_str)
                .collect::<Vec<_>>()
                .join(", "),
            Self::UniqueLang(value) | Self::SingleLine(value) | Self::UniqueMembers(value) => {
                value.to_string()
            }
            Self::Equals(path)
            | Self::Disjoint(path)
            | Self::SubsetOf(path)
            | Self::LessThan(path)
            | Self::LessThanOrEquals(path) => path.to_string(),
            Self::And(iris) | Self::Or(iris) | Self::Xone(iris) => iris
                .iter()
                .map(|value| format!("{value:?}"))
                .collect::<Vec<_>>()
                .join(", "),
            Self::QualifiedValueShape {
                shape,
                min_count,
                max_count,
                ..
            } => {
                let mut parts = vec![format!("{shape:?}")];
                if let Some(min) = min_count {
                    parts.push(format!("min={min:?}"));
                }
                if let Some(max) = max_count {
                    parts.push(format!("max={max:?}"));
                }
                parts.join(", ")
            }
            Self::In(values) => values
                .iter()
                .map(|value| format!("{value:?}"))
                .collect::<Vec<_>>()
                .join(", "),
            Self::Closed {
                closedness,
                ignored_properties,
            } => {
                if closedness.is_closed() && !ignored_properties.is_empty() {
                    let ignored = ignored_properties
                        .iter()
                        .map(crate::local_name)
                        .collect::<Vec<_>>();
                    format!("{} (ignore: {})", closedness.as_str(), ignored.join(", "))
                } else {
                    closedness.as_str().to_owned()
                }
            }
            Self::Sparql(constraint) => format!("{:?}", constraint.select.text),
            Self::Expression(constraint) => format!("{:?}", constraint.expression.node),
            Self::Custom(constraint) => crate::local_name(&constraint.component),
            Self::ReifierShape {
                shape,
                reification_required,
            } => match reification_required {
                Some(required) => format!("{shape:?}, required={required}"),
                None => format!("{shape:?}"),
            },
            Self::RootClass(named) | Self::UniqueValuesFor(named) => named
                .iter()
                .map(crate::local_name)
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}
