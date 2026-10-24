//! Portable list declarations from SHACL 1.2 Core, Working Draft 2026-10-14.

use rdf_structural_model::{BlankNodeId, Literal};
use serde::{Deserialize, Serialize};
use shacl_structural_model::{Component, DefaultTerm, NodeShape, PropertyShape, ShaclPath};

fn roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(value: &T) {
    let json = serde_json::to_vec(value).unwrap();
    assert_eq!(&serde_json::from_slice::<T>(&json).unwrap(), value);
    let binary = postcard::to_allocvec(value).unwrap();
    assert_eq!(&postcard::from_bytes::<T>(&binary).unwrap(), value);
}

fn blank(label: &str) -> DefaultTerm {
    DefaultTerm::BlankNode(
        BlankNodeId::try_new("urn:shapes".parse().unwrap(), label.to_owned()).unwrap(),
    )
}

#[test]
fn nested_shapes_preserve_list_references_large_lexical_counts_and_both_booleans() {
    let count = DefaultTerm::literal(
        Literal::typed(
            "+000340282366920938463463374607431768211456",
            "http://www.w3.org/2001/XMLSchema#integer".parse().unwrap(),
        )
        .unwrap(),
    );
    for unique in [true, false] {
        let components = [
            Component::MemberShape(blank("member")),
            Component::MinListLength(count.clone()),
            Component::MaxListLength(count.clone()),
            Component::UniqueMembers(unique),
        ];
        let mut node = NodeShape::<DefaultTerm>::new(blank("node"));
        let mut property = PropertyShape::<DefaultTerm>::new(
            blank("property"),
            ShaclPath::Predicate("urn:items".parse().unwrap()),
        );
        for component in components {
            node.components.push(component.clone());
            property.components.push(component);
        }
        assert_eq!(node.components[0].kind_name(), "memberShape");
        assert_eq!(node.components[1].kind_name(), "minListLength");
        assert_eq!(node.components[2].kind_name(), "maxListLength");
        assert_eq!(node.components[3].kind_name(), "uniqueMembers");
        assert_eq!(node.components[3].display_value(), unique.to_string());
        roundtrip(&(node, property));
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CompiledCount([u64; 2]);

#[test]
fn list_lengths_support_independent_compiled_counts() {
    let constraints: [Component<String, CompiledCount, u8, String, String>; 2] = [
        Component::MinListLength(CompiledCount([1, 0])),
        Component::MaxListLength(CompiledCount([u64::MAX, u64::MAX])),
    ];
    assert!(constraints[0].display_value().contains("CompiledCount"));
    assert!(constraints[1].display_value().contains("CompiledCount"));
    roundtrip(&constraints);
}

#[test]
fn list_declarations_do_not_require_cloning_or_serialization_for_construction() {
    struct Opaque;
    type Constraint = Component<Opaque, Opaque, Opaque, Opaque, Opaque>;
    for constraint in [
        Constraint::MemberShape(Opaque),
        Constraint::MinListLength(Opaque),
        Constraint::MaxListLength(Opaque),
        Constraint::UniqueMembers(false),
    ] {
        assert_ne!(constraint.kind_name(), "");
    }
}

#[test]
fn unique_members_rejects_non_boolean_parameters_in_both_formats() {
    for invalid in [r#"{"UniqueMembers":1}"#, r#"{"UniqueMembers":"true"}"#] {
        assert!(serde_json::from_str::<Component<String>>(invalid).is_err());
    }
    let mut invalid = postcard::to_allocvec(&Component::<String>::UniqueMembers(true)).unwrap();
    *invalid.last_mut().unwrap() = 2;
    assert!(postcard::from_bytes::<Component<String>>(&invalid).is_err());
}
