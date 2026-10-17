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

fn round_trip<T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug>(
    value: &T,
) {
    let json = serde_json::to_string(value).unwrap();
    let binary = postcard::to_allocvec(value).unwrap();
    assert_eq!(&serde_json::from_str::<T>(&json).unwrap(), value);
    assert_eq!(&postcard::from_bytes::<T>(&binary).unwrap(), value);
}

fn blank(scope: &str, label: &str) -> DefaultTerm {
    DefaultTerm::BlankNode(
        rdf_structural_model::BlankNodeId::try_new(scope.parse().unwrap(), label.to_owned())
            .unwrap(),
    )
}

fn text(value: &str) -> DefaultTerm {
    DefaultTerm::literal(rdf_structural_model::Literal::string(value))
}

#[test]
fn default_shapes_preserve_scopes_directional_messages_and_numeric_lexicals() {
    let label = DefaultTerm::literal(rdf_structural_model::Literal::directional(
        "Required",
        rdf_structural_model::LanguageTag::try_new("en-US".to_owned()).unwrap(),
        rdf_structural_model::BaseDirection::Ltr,
    ));
    let count = DefaultTerm::literal(
        rdf_structural_model::Literal::typed(
            "+000184467440737095516160",
            "http://www.w3.org/2001/XMLSchema#integer".parse().unwrap(),
        )
        .unwrap(),
    );
    let node =
        shacl_structural_model::NodeShape::<DefaultTerm>::new(blank("urn:document:a", "shape"))
            .with_property_shape(blank("urn:document:a", "property"))
            .with_message(label.clone())
            .with_component(Component::MinCount(count))
            .with_component(Component::Pattern {
                pattern: "^a".to_owned(),
                flags: Some(text("i")),
            })
            .with_rule(shacl_structural_model::ShaclRule::new(
                blank("urn:document:a", "rule"),
                shacl_structural_model::RuleKind::Sparql(shacl_structural_model::SparqlQuery::new(
                    text("CONSTRUCT {} WHERE {}"),
                )),
            ));
    let property = shacl_structural_model::PropertyShape::<DefaultTerm>::new(
        blank("urn:document:a", "property"),
        shacl_structural_model::ShaclPath::Predicate("urn:name".parse().unwrap()),
    )
    .with_name(label)
    .with_order(text("+0001.00000000000000001"));
    round_trip(&(node, property));
    assert_ne!(
        blank("urn:document:a", "shape"),
        blank("urn:document:b", "shape")
    );
}

#[test]
fn nested_triple_terms_survive_default_shape_targets() {
    let triple = rdf_structural_model::Triple::new(
        rdf_structural_model::Subject::Iri("urn:subject".parse().unwrap()),
        "urn:predicate".parse().unwrap(),
        text("exact object"),
    );
    let shape =
        shacl_structural_model::NodeShape::<DefaultTerm>::new(blank("urn:document", "shape"))
            .with_target(shacl_structural_model::Target::Node(DefaultTerm::triple(
                triple,
            )));
    round_trip(&shape);
}

#[test]
fn default_shape_terms_support_borrowed_binary_decoding() {
    let shape =
        shacl_structural_model::NodeShape::<DefaultTerm>::new(blank("urn:document", "shape"))
            .with_message(text("borrowed message"));
    let bytes = postcard::to_allocvec(&shape).unwrap();
    let decoded: shacl_structural_model::NodeShape<rdf_structural_model::Term<&str>> =
        postcard::from_bytes(&bytes).unwrap();
    let rdf_structural_model::Term::Literal(message) = &decoded.messages[0] else {
        panic!("message must stay a literal")
    };
    assert_eq!(message.lexical(), "borrowed message");
    assert!(message.lexical().as_ptr() >= bytes.as_ptr());
    assert!(message.lexical().as_ptr() < bytes.as_ptr().wrapping_add(bytes.len()));
}
