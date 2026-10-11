//! Count parameters must preserve the caller's complete integer representation.

use shacl_structural_model::{Component, NodeShape, PropertyShape, ShaclPath};

#[test]
fn count_parameters_preserve_integer_lexicals() {
    for kind in ["MinCount", "MaxCount", "MinLength", "MaxLength"] {
        for lexical in ["-1", "18446744073709551616", "+00001"] {
            let encoded = format!("{{\"{kind}\":\"{lexical}\"}}");
            let component: Component<String> = serde_json::from_str(&encoded)
                .expect("count parameters must preserve caller values instead of requiring u64");
            assert_eq!(
                component.kind_name().to_ascii_lowercase(),
                kind.to_ascii_lowercase()
            );
            assert_eq!(serde_json::to_string(&component).unwrap(), encoded);
        }
    }
}

#[test]
fn qualified_counts_preserve_integer_lexicals() {
    let encoded = r#"{"QualifiedValueShape":{"shape":"urn:qualified","min_count":"-1","max_count":"18446744073709551616","disjoint":false}}"#;
    let component: Component<String> =
        serde_json::from_str(encoded).expect("qualified counts must preserve full integer values");
    assert_eq!(serde_json::to_string(&component).unwrap(), encoded);
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
enum CompiledCount {
    BelowZero,
    Finite(u64),
    AboveU64,
}

#[test]
fn shapes_support_an_independent_compiled_count_type() {
    let component = Component::<String, CompiledCount>::MinCount(CompiledCount::AboveU64);
    let node = NodeShape::<String, CompiledCount>::new("urn:node".parse().unwrap())
        .with_component(component.clone());
    let property = PropertyShape::<String, CompiledCount>::new(
        "urn:property".parse().unwrap(),
        ShaclPath::Predicate("urn:path".parse().unwrap()),
    )
    .with_component(component);
    let encoded = serde_json::to_string(&(node.clone(), property.clone())).unwrap();
    let decoded: (
        NodeShape<String, CompiledCount>,
        PropertyShape<String, CompiledCount>,
    ) = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded, (node, property));
}
