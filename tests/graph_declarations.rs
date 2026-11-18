//! Graph roles, graph links, entailment and incomplete-data declarations from
//! SHACL 1.2 Core, Working Draft 16 November 2026 (sections 1.4, 6.1, 6.2, 6.5,
//! 6.6 and 6.9.3.1), and Inference Rules, Working Draft 10 November 2026
//! (sections 5 and 6).

#[path = "support/golden.rs"]
mod golden;

use iri_rfc3987::Iri;
use proptest::prelude::*;
use shacl_structural_model::graph::{
    Entailment, EntailmentDeclaration, GraphLink, GraphLinkKind, GraphRole, GraphTypeDeclaration,
    IncompleteNode, IncompleteUriStart,
};
use shacl_structural_model::{DefaultTerm, vocabulary};

fn iri(local: &str) -> Iri<String> {
    format!("http://example.com/{local}").parse().unwrap()
}

const ROLES: [GraphRole; 4] = [
    GraphRole::Graph,
    GraphRole::DataGraph,
    GraphRole::ShapesGraph,
    GraphRole::RulesGraph,
];

#[test]
fn graph_roles_and_links_are_streamable_triples() {
    for role in ROLES {
        assert_eq!(GraphRole::from_class_iri(role.class_iri()), Some(role));
    }
    assert_eq!(GraphRole::from_class_iri(vocabulary::SHAPES_GRAPH), None);
    assert!(GraphRole::RulesGraph.is_shapes_graph());
    assert!(!GraphRole::DataGraph.is_shapes_graph());
    assert!(GraphLinkKind::ShapesGraph.follows_imports());
    assert!(!GraphLinkKind::DataGraph.follows_imports());
    assert_eq!(
        GraphLinkKind::SuggestedShapesGraph.predicate_iri(),
        vocabulary::SUGGESTED_SHAPES_GRAPH
    );
    let records = (
        GraphTypeDeclaration::new(iri("myDataGraph"), GraphRole::DataGraph),
        GraphLink::new(
            iri("myDataGraph"),
            GraphLinkKind::ShapesGraph,
            iri("graph-shapes1"),
        ),
        GraphLink::new(
            iri("myShapesGraph"),
            GraphLinkKind::DataGraph,
            iri("bridges-2026"),
        ),
        EntailmentDeclaration::new(iri("my-shapes"), Entailment::Rules),
        EntailmentDeclaration::new(iri("my-shapes"), Entailment::Regime(iri("RuleSet2"))),
    );
    golden::assert_golden("core12_graphs", &records);
}

#[test]
fn rules_entailment_is_classified_from_its_iri() {
    let rules: Iri<String> = vocabulary::RULES_ENTAILMENT.parse().unwrap();
    assert_eq!(Entailment::from_iri(rules), Entailment::Rules);
    let rdfs: Iri<String> = "http://www.w3.org/ns/entailment/RDFS".parse().unwrap();
    let regime = Entailment::from_iri(rdfs.clone());
    assert_eq!(regime, Entailment::Regime(rdfs));
    assert_eq!(regime.iri(), "http://www.w3.org/ns/entailment/RDFS");
    assert_eq!(
        Entailment::<Iri<String>>::Rules.iri(),
        vocabulary::RULES_ENTAILMENT
    );
}

#[test]
fn incomplete_data_declarations_keep_explicit_booleans() {
    let companies = IncompleteUriStart::new(iri("companies/"), true);
    assert!(companies.covers("http://example.com/companies/ACME"));
    assert!(!companies.covers("http://example.com/companies/"));
    assert!(!companies.covers("http://example.com/people/Alice"));
    assert!(
        !IncompleteUriStart::new(iri("companies/"), false).covers("http://example.com/companies/x")
    );
    let globex =
        IncompleteNode::<DefaultTerm>::new(DefaultTerm::iri(iri("companies/Globex")), false);
    golden::assert_golden("core12_incomplete", &(companies, globex));
    assert!(
        serde_json::from_str::<IncompleteNode>(
            r#"{"node":{"Iri":"urn:a"},"incomplete":true,"graph":null}"#
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<GraphLink>(
            r#"{"from":"urn:a","kind":"DataGraph","to":"urn:b","imports":true}"#
        )
        .is_err()
    );
}

proptest! {
    #[test]
    fn graph_records_round_trip(
        from in "[a-z]{1,6}",
        to in "[a-z]{1,6}",
        role in prop::sample::select(ROLES.to_vec()),
        kind in prop::sample::select(vec![
            GraphLinkKind::ShapesGraph,
            GraphLinkKind::DataGraph,
            GraphLinkKind::SuggestedShapesGraph,
        ]),
        flag in any::<bool>(),
    ) {
        golden::round_trip(&GraphTypeDeclaration::new(iri(&from), role));
        golden::round_trip(&GraphLink::new(iri(&from), kind, iri(&to)));
        golden::round_trip(&EntailmentDeclaration::new(iri(&from), Entailment::Regime(iri(&to))));
        golden::round_trip(&IncompleteUriStart::new(iri(&from), flag));
        golden::round_trip(&IncompleteNode::<DefaultTerm>::new(DefaultTerm::iri(iri(&to)), flag));
    }
}
