//! Structural coverage of the SHACL 1.2 Core 2026-10-14 singleLine draft.

use shacl_structural_model::{Component, NodeShape, PropertyShape, ShaclPath};

#[test]
fn single_line_preserves_true_and_false_through_nested_text_and_binary() {
    for enabled in [true, false] {
        let node = NodeShape::<String>::new("urn:node".to_owned())
            .with_component(Component::SingleLine(enabled));
        let property = PropertyShape::<String>::new(
            "urn:property".to_owned(),
            ShaclPath::Predicate("urn:label".parse().unwrap()),
        )
        .with_component(Component::SingleLine(enabled));
        let original = (node, property);
        let json = serde_json::to_vec(&original).unwrap();
        let restored: (NodeShape<String>, PropertyShape<String>) =
            serde_json::from_slice(&json).unwrap();
        assert_eq!(restored, original);
        let binary = postcard::to_allocvec(&original).unwrap();
        let restored: (NodeShape<String>, PropertyShape<String>) =
            postcard::from_bytes(&binary).unwrap();
        assert_eq!(restored, original);
        assert_eq!(original.0.components[0].kind_name(), "singleLine");
        assert_eq!(
            original.1.components[0].display_value(),
            enabled.to_string()
        );
    }
}

#[test]
fn single_line_rejects_non_boolean_parameters() {
    for invalid in [
        r#"{"SingleLine":"true"}"#,
        r#"{"SingleLine":1}"#,
        r#"{"SingleLine":null}"#,
    ] {
        assert!(serde_json::from_str::<Component<String>>(invalid).is_err());
    }
    // The binary variant discriminant comes from the serializer, not a fixed tag.
    let mut invalid = postcard::to_allocvec(&Component::<String>::SingleLine(true)).unwrap();
    *invalid.last_mut().unwrap() = 2;
    assert!(postcard::from_bytes::<Component<String>>(&invalid).is_err());
}
