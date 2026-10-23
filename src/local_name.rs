//! IRI display helpers.

/// Return the local name after the final `#` or `/` in an IRI.
#[must_use]
pub fn local_name(iri: &(impl AsRef<str> + ?Sized)) -> String {
    let value = iri.as_ref();
    if let Some(position) = value.rfind('#') {
        value[position + 1..].to_owned()
    } else if let Some(position) = value.rfind('/') {
        value[position + 1..].to_owned()
    } else {
        value.to_owned()
    }
}
