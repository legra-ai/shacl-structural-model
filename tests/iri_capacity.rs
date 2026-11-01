//! SHACL paths and shared RDF terms expose their retained IRI allocation.

use iri_rfc3987::Iri;
use rdf_structural_model::Literal;
use shacl_structural_model::{Component, DefaultTerm, ShaclPath};

#[test]
fn path_and_literal_iris_expose_their_retained_capacity() {
    let mut predicate_text = String::with_capacity(4096);
    predicate_text.push_str("urn:predicate:allocated");
    let predicate_capacity = predicate_text.capacity();
    let path: ShaclPath = ShaclPath::Predicate(Iri::parse_owned(predicate_text).unwrap());
    let ShaclPath::Predicate(predicate) = path else {
        panic!("expected predicate path");
    };
    let predicate: &Iri<String> = &predicate;
    assert_eq!(predicate.capacity(), predicate_capacity);

    let mut datatype_text = String::with_capacity(8192);
    datatype_text.push_str("urn:datatype:allocated");
    let datatype_capacity = datatype_text.capacity();
    let literal = Literal::typed("value", Iri::parse_owned(datatype_text).unwrap()).unwrap();
    let datatype: &Iri<String> = literal.metadata().datatype();
    assert_eq!(datatype.capacity(), datatype_capacity);
    let component: Component = Component::HasValue(DefaultTerm::literal(literal));
    assert_eq!(component.kind_name(), "hasValue");
}
