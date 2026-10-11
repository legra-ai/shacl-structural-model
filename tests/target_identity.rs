//! Blank-node target identities remain opaque caller-supplied RDF terms.

use oxrdf::{BlankNode, Literal, Term};
use shacl_structural_model::{CustomTarget, NodeShape, SparqlQuery, SparqlTargetType, Target};

#[test]
fn implicit_class_and_target_type_references_keep_blank_node_identity() {
    let class: Term = BlankNode::new("class").unwrap().into();
    let target_type: Term = BlankNode::new("target-type").unwrap().into();
    let query = SparqlQuery::new(Literal::new_simple_literal("SELECT ?this WHERE {}").into());
    let definition = SparqlTargetType::<Term>::new(target_type.clone(), query);
    let shape = NodeShape::<Term>::new(class.clone())
        .with_target(Target::ImplicitClass(class.clone()))
        .with_target(Target::Custom(CustomTarget {
            id: BlankNode::new("instance").unwrap().into(),
            target_type: definition.id,
            parameters: Vec::new(),
        }));
    assert_eq!(shape.targets[0], Target::ImplicitClass(class));
    assert!(
        matches!(&shape.targets[1], Target::Custom(target) if target.target_type == target_type)
    );
}
