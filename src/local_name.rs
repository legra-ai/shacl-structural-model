//! IRI display helpers.

use iri_rfc3987::Iri;

/// Return the local name after the final `#` or `/` in an IRI.
#[must_use]
pub fn local_name(iri: &Iri<String>) -> String {
    let value = iri.as_str();
    if let Some(position) = value.rfind('#') {
        value[position + 1..].to_owned()
    } else if let Some(position) = value.rfind('/') {
        value[position + 1..].to_owned()
    } else {
        value.to_owned()
    }
}
