//! Use actual RDF term implementations at the model/engine boundary.

use oxrdf::{BlankNode, Literal, NamedNode, Term};
use shacl_structural_model::{
    Component, DefaultTerm, NodeShape, PropertyShape, ShaclPath, SparqlConstraint, SparqlQuery,
    Target,
};

#[test]
fn oxigraph_terms_move_directly_into_structural_values() {
    let id: Term = BlankNode::new("shape").unwrap().into();
    let property: Term = BlankNode::new("property").unwrap().into();
    let message: Term = Literal::new_language_tagged_literal("Verplicht", "nl")
        .unwrap()
        .into();
    let focus: Term = Literal::new_typed_literal(
        "+0009007199254740993",
        NamedNode::new("http://www.w3.org/2001/XMLSchema#integer").unwrap(),
    )
    .into();
    let mut sparql = SparqlConstraint::new(
        BlankNode::new("sparql").unwrap().into(),
        SparqlQuery::new(Literal::new_simple_literal("SELECT $this WHERE {}").into()),
    );
    sparql.messages.push(message.clone());
    let shape = NodeShape::<Term>::new(id.clone())
        .with_property_shape(property.clone())
        .with_target(Target::Node(focus.clone()))
        .with_message(message.clone())
        .with_component(Component::Node(property.clone()))
        .with_component(Component::Sparql(Box::new(sparql)));
    assert_eq!(shape.id, id);
    assert_eq!(shape.property_shapes, [property]);
    assert_eq!(shape.messages, [message]);
    assert_eq!(shape.targets, [Target::Node(focus)]);
}

#[test]
fn borrowed_engine_terms_need_no_owned_conversion() {
    let id: Term = BlankNode::new("shape").unwrap().into();
    let message: Term = Literal::new_language_tagged_literal("Name", "en")
        .unwrap()
        .into();
    let order: Term = Literal::new_typed_literal(
        "+0001.0000000000000000001",
        NamedNode::new("http://www.w3.org/2001/XMLSchema#decimal").unwrap(),
    )
    .into();
    let shape =
        PropertyShape::<&Term, i128>::new(&id, ShaclPath::Predicate("urn:name".parse().unwrap()))
            .with_target(Target::Node(&message))
            .with_message(&message)
            .with_name(&message)
            .with_description(&message)
            .with_order(&order)
            .with_component(Component::MinCount(i128::MAX));
    assert!(std::ptr::eq(shape.id, &raw const id));
    assert!(std::ptr::eq(shape.messages[0], &raw const message));
    assert!(std::ptr::eq(shape.names[0], &raw const message));
    assert!(std::ptr::eq(shape.order.unwrap(), &raw const order));
}

#[test]
fn default_rdf_terms_round_trip_blank_ids_and_language_messages() {
    let id = DefaultTerm::from(rdf_syntax::BlankIdBuf::new("_:shape".to_owned()).unwrap());
    let message = DefaultTerm::literal(rdf_syntax::Literal::new(
        "Verplicht",
        rdf_syntax::LiteralType::LangString("nl".parse().unwrap()),
    ));
    let shape = NodeShape::<DefaultTerm>::new(id)
        .with_message(message.clone())
        .with_target(Target::Node(message));
    let encoded = serde_json::to_string(&shape).unwrap();
    let decoded: NodeShape = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, shape);
}
