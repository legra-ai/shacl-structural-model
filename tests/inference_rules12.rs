//! Rule metadata, rule types, SPARQL rules, rule templates, processors and
//! inferred-triple reifiers from SHACL 1.2 Inference Rules, Working Draft 10
//! November 2026 (sections 3, 4, 7, 9, 10, 11 and 12).

#[path = "support/golden.rs"]
mod golden;

use iri_rfc3987::Iri;
use proptest::prelude::*;
use rdf_structural_model::{Literal, Subject, Term, Triple};
use shacl_structural_model::inference::{
    ConstructQuery, INFERENCE_DRAFT, InferenceAnnotation, InferenceModelError,
    InferredTripleReifier, RuleExpression, RuleMetadata, RuleNumber, RuleProcessor, RuleSchedule,
    RuleSet, RuleType, RuleTyping, SparqlRuleDeclaration, SparqlRuleTemplate, TemplateParameter,
    TemplateRuleDeclaration, TripleRuleDeclaration,
};
use shacl_structural_model::{ParameterValues, vocabulary};

fn iri(local: &str) -> Iri<String> {
    format!("http://example.com/ns#{local}").parse().unwrap()
}

fn integer(value: &str) -> RuleNumber {
    RuleNumber::try_from_literal(
        Literal::typed(
            value,
            "http://www.w3.org/2001/XMLSchema#integer".parse().unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

fn construct(text: &str) -> ConstructQuery {
    ConstructQuery::try_new(Literal::string(text)).unwrap()
}

/// Inference Rules example 7: a rule expecting derived `ex:area` triples.
fn compute_small() -> SparqlRuleDeclaration {
    let metadata = RuleMetadata::new()
        .with_schedule(
            RuleSchedule::new()
                .with_layer(integer("1"))
                .with_run_once(true),
        )
        .with_condition(Subject::Iri(iri("RectangleShape")))
        .with_deactivated(false)
        .with_expected_predicate(iri("area"))
        .with_processor(RuleProcessor::Literal("SRL 1.2".to_owned()))
        .with_processor(RuleProcessor::Iri(iri("Datalog")));
    SparqlRuleDeclaration::new(
        Subject::Iri(iri("Rectangle-computeSmall")),
        construct("CONSTRUCT { $this ex:isSmall true . } WHERE { $this ex:area ?area }")
            .with_prefixes(Subject::Iri(iri(""))),
    )
    .with_metadata(metadata)
}

/// Inference Rules example 14: a symmetric-property rule template and an instance.
fn symmetric() -> (SparqlRuleTemplate, TemplateRuleDeclaration) {
    let parameter = TemplateParameter::new(Term::Iri(iri("property-parameter")), iri("property"));
    let template = SparqlRuleTemplate::new(
        iri("SymmetricPropertyRule"),
        construct("CONSTRUCT { ?o $property ?s . } WHERE { ?s $property ?o . }"),
    )
    .with_parameter(parameter);
    let rule = TemplateRuleDeclaration::new(
        Subject::Iri(iri("MarriedToRule")),
        iri("SymmetricPropertyRule"),
    )
    .with_argument(ParameterValues {
        path: iri("property"),
        values: vec![Term::Iri(iri("marriedTo"))],
    });
    (template, rule)
}

#[test]
fn rules_retain_conditions_deactivation_expected_predicates_and_processors() {
    let rule = compute_small();
    let metadata = rule.metadata();
    assert_eq!(metadata.conditions(), [Subject::Iri(iri("RectangleShape"))]);
    assert_eq!(metadata.deactivated(), Some(false));
    assert!(!metadata.is_deactivated());
    assert_eq!(metadata.expected_predicates(), [iri("area")]);
    assert_eq!(metadata.processors()[0].as_str(), "SRL 1.2");
    assert_eq!(metadata.schedule().run_once(), Some(true));
    assert_eq!(RuleMetadata::<String>::new().deactivated(), None);
    assert!(
        RuleMetadata::<String>::new()
            .with_deactivated(true)
            .is_deactivated()
    );
    let set = RuleSet::new(iri("RuleSet1")).with_processor(RuleProcessor::Iri(iri("Datalog")));
    assert_eq!(set.processors().len(), 1);
    let triple = TripleRuleDeclaration::new(Subject::Iri(iri("childCount")))
        .with_metadata(RuleMetadata::new().with_expected_predicate(iri("child")))
        .with_predicate(RuleExpression::constant(Term::Iri(iri("childCount"))));
    assert_eq!(triple.metadata().expected_predicates(), [iri("child")]);
    golden::assert_golden("inference12_rules", &(rule, set, triple));
    assert!(INFERENCE_DRAFT.ends_with("WD-shacl12-inference-rules-20261110/"));
}

#[test]
fn rule_types_classify_built_in_and_template_iris() {
    let sparql: Iri<String> = vocabulary::SPARQL_RULE.parse().unwrap();
    let triple: Iri<String> = vocabulary::TRIPLE_RULE.parse().unwrap();
    assert_eq!(RuleType::from_iri(sparql), RuleType::Sparql);
    assert_eq!(RuleType::from_iri(triple), RuleType::Triple);
    let template = RuleType::from_iri(iri("SymmetricPropertyRule"));
    assert!(!template.is_built_in());
    assert_eq!(template.iri(), iri("SymmetricPropertyRule").as_str());
    assert_eq!(RuleType::<String>::Sparql.iri(), vocabulary::SPARQL_RULE);
    let typing = RuleTyping::new(Subject::Iri(iri("MarriedToRule")), template);
    assert_eq!(
        typing.rule_type().iri(),
        iri("SymmetricPropertyRule").as_str()
    );
    golden::round_trip(&typing);
}

#[test]
fn rule_templates_declare_parameters_and_instances_supply_arguments() {
    let (template, rule) = symmetric();
    assert_eq!(template.required_parameters().count(), 1);
    assert_eq!(rule.template(), template.id());
    assert_eq!(rule.arguments()[0].values, [Term::Iri(iri("marriedTo"))]);
    assert!(template.query().text().starts_with("CONSTRUCT"));
    golden::assert_golden("inference12_template", &(template, rule));
}

#[test]
fn construct_queries_must_be_string_literals_on_construction_and_decoding() {
    let typed = Literal::typed(
        "1",
        "http://www.w3.org/2001/XMLSchema#integer".parse().unwrap(),
    )
    .unwrap();
    assert_eq!(
        ConstructQuery::try_new(typed.clone()),
        Err(InferenceModelError::InvalidConstructQuery)
    );
    let valid = serde_json::to_value(construct("CONSTRUCT {} WHERE {}")).unwrap();
    let mut invalid = valid.clone();
    invalid["construct"] = serde_json::to_value(&typed).unwrap();
    assert!(serde_json::from_value::<ConstructQuery>(invalid).is_err());
    let mut unknown = valid;
    unknown["select"] = serde_json::Value::Null;
    assert!(serde_json::from_value::<ConstructQuery>(unknown).is_err());
    assert!(
        serde_json::from_str::<RuleMetadata>(
            r#"{"schedule":{"layer":null,"order":null,"run_once":null},"conditions":[],"deactivated":null,"expected_predicates":[],"processors":[],"engine":null}"#
        )
        .is_err()
    );
}

#[test]
fn reifiers_mark_temporary_triples_and_source_rules() {
    let triple = Triple::new(
        Subject::Iri(iri("Alice")),
        iri("friend"),
        Term::Iri(iri("Bob")),
    );
    let temporary = InferredTripleReifier::new(
        Subject::Iri(iri("r1")),
        triple.clone(),
        InferenceAnnotation::TempTriple(true),
    );
    assert!(temporary.is_temporary());
    let source = InferredTripleReifier::new(
        Subject::Iri(iri("r2")),
        triple,
        InferenceAnnotation::SourceRule(Subject::Iri(iri("SymmetricPropertyRule"))),
    );
    assert!(!source.is_temporary());
    golden::assert_golden("inference12_reifiers", &(temporary, source));
}

fn processor() -> impl Strategy<Value = RuleProcessor> {
    prop_oneof![
        "[a-z]{1,6}".prop_map(|local| RuleProcessor::Iri(iri(&local))),
        ".{0,8}".prop_map(RuleProcessor::Literal),
    ]
}

proptest! {
    #[test]
    fn rule_declarations_round_trip(
        conditions in prop::collection::vec("[a-z]{1,6}", 0..3),
        deactivated in prop::option::of(any::<bool>()),
        expected in prop::collection::vec("[a-z]{1,6}", 0..3),
        processors in prop::collection::vec(processor(), 0..3),
        text in ".{0,16}",
        temporary in any::<bool>(),
    ) {
        let mut metadata = RuleMetadata::new();
        for condition in &conditions {
            metadata = metadata.with_condition(Subject::Iri(iri(condition)));
        }
        for predicate in &expected {
            metadata = metadata.with_expected_predicate(iri(predicate));
        }
        for processor in processors {
            metadata = metadata.with_processor(processor);
        }
        if let Some(deactivated) = deactivated {
            metadata = metadata.with_deactivated(deactivated);
        }
        let rule = SparqlRuleDeclaration::new(Subject::Iri(iri("rule")), construct(&text))
            .with_metadata(metadata.clone());
        golden::round_trip(&rule);
        let instance = TemplateRuleDeclaration::new(Subject::Iri(iri("rule")), iri("template"))
            .with_metadata(metadata);
        golden::round_trip(&instance);
        golden::round_trip(&InferredTripleReifier::new(
            Subject::Iri(iri("r")),
            Triple::new(Subject::Iri(iri("s")), iri("p"), Term::literal(Literal::string(text))),
            InferenceAnnotation::<String>::TempTriple(temporary),
        ));
    }
}
