//! The default RDF term must work with non-self-describing formats.

use shacl_structural_model::{Component, DefaultTerm};

#[test]
fn default_literal_round_trips_in_binary() {
    let component: Component = Component::HasValue(DefaultTerm::literal(
        rdf_structural_model::Literal::string("005"),
    ));
    let bytes = postcard::to_allocvec(&component).unwrap();
    assert_eq!(
        postcard::from_bytes::<Component>(&bytes).unwrap(),
        component
    );
}
