//! A parameter whose value is one IRI or a SHACL list of IRIs.

/// A single named value or a SHACL list of named values.
///
/// SHACL 1.2 Core, Working Draft 16 November 2026, sections 6.10.9.4
/// (`sh:rootClass`) and 6.10.9.5 (`sh:uniqueValuesFor`) accept either form.
/// The form is retained so a single IRI and a one-member list stay distinct.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum NamedOrList<Named> {
    /// A single IRI value.
    One(Named),
    /// A SHACL list whose members are IRIs.
    List(Vec<Named>),
}

impl<Named> NamedOrList<Named> {
    /// Iterate over the named members, in declared order.
    pub fn iter(&self) -> std::slice::Iter<'_, Named> {
        match self {
            Self::One(named) => std::slice::from_ref(named).iter(),
            Self::List(members) => members.iter(),
        }
    }

    /// Return the number of named members.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::One(_) => 1,
            Self::List(members) => members.len(),
        }
    }

    /// Whether this is an empty SHACL list.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether the value was written as a SHACL list.
    #[must_use]
    pub const fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }
}

impl<'a, Named> IntoIterator for &'a NamedOrList<Named> {
    type Item = &'a Named;
    type IntoIter = std::slice::Iter<'a, Named>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
