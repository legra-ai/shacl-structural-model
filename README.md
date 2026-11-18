# shacl-structural-model

[![crates.io](https://img.shields.io/crates/v/shacl-structural-model.svg)](https://crates.io/crates/shacl-structural-model)
[![Documentation](https://docs.rs/shacl-structural-model/badge.svg)](https://docs.rs/shacl-structural-model)
[![CI](https://github.com/legra-ai/shacl-structural-model/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/shacl-structural-model/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/shacl-structural-model.svg)](https://github.com/legra-ai/shacl-structural-model#license)

Typed structural values for SHACL shapes in Rust.

`shacl-structural-model` represents SHACL shapes, property paths, targets,
constraint components, severities, RDF messages, SPARQL declarations, and
SHACL Advanced Features functions, expressions and rules. It is intended for parsers,
editors, validators, serializers, and storage adapters that need typed shape
values without taking a dependency on an RDF graph implementation.

The default value type is [`rdf_structural_model::Term`]. The structural types are also
generic over the value type, so an application can use its own RDF term model
without copying or converting RDF-valued fields. Shape IDs and references
retain blank nodes; target nodes can also be literals. Messages keep language
tags and datatypes. Numeric ordering literals retain their lexical precision.

This crate models structural values only. It does not parse RDF, read or
write files, access a graph store, execute validation, run SPARQL, or collect
an entire shapes graph. Callers should stream shapes from their input and
retain only the structural values needed by their operation.

## W3C specifications

The stable baseline is the [SHACL Recommendation](https://www.w3.org/TR/shacl/).
SPARQL and AF declarations follow that recommendation and
[SHACL Advanced Features](https://www.w3.org/TR/shacl-af/).
Representing these declarations does not establish validator conformance or
pySHACL compatibility. SHACL 1.2 is the target specification, tracked at its
newest drafts: each draft feature is added as a model-only declaration as it is
modelled, and the declarations listed in this README are the ones modelled so
far. SHACL-JS is outside the scope.

### SHACL 1.2 Core declarations

These follow [SHACL 1.2 Core, Working Draft 16 November 2026](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/). The crate
represents each declaration; it does not evaluate it.

| Declaration | Model | Draft section |
| --- | --- | --- |
| `sh:Debug`, `sh:Trace` severities | `Severity::Debug`, `Severity::Trace` | [3.1.4](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#severity) |
| `sh:singleLine` (at risk) | `Component::SingleLine(bool)` | [6.10.4.4](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#SingleLineConstraintComponent) |
| `sh:memberShape` | `Component::MemberShape(Value)` | [6.10.5.1](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#MemberShapeConstraintComponent) |
| `sh:minListLength` | `Component::MinListLength(Count)` | [6.10.5.2](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#MinListLengthConstraintComponent) |
| `sh:maxListLength` | `Component::MaxListLength(Count)` | [6.10.5.3](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#MaxListLengthConstraintComponent) |
| `sh:uniqueMembers` | `Component::UniqueMembers(bool)` | [6.10.5.4](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#UniqueMembersConstraintComponent) |
| `sh:subsetOf` | `Component::SubsetOf(ShaclPath<Named>)` | [6.10.6.3](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#SubsetOfConstraintComponent) |
| `sh:someValue` | `Component::SomeValue(Value)` | [6.10.8.3](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#SomeValueConstraintComponent) |
| `sh:reifierShape`, `sh:reificationRequired` | `Component::ReifierShape { shape, reification_required }` | [6.10.8.5](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#ReifierShapeConstraintComponent) |
| `sh:closed sh:ByTypes` | `Component::Closed { closedness: Closedness::ByTypes, .. }` | [6.10.9.1](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#ClosedConstraintComponent) |
| `sh:rootClass` | `Component::RootClass(NamedOrList<Named>)` | [6.10.9.4](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#RootClassConstraintComponent) |
| `sh:uniqueValuesFor` | `Component::UniqueValuesFor(NamedOrList<Named>)` | [6.10.9.5](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#UniqueValuesForConstraintComponent) |

`SingleLine` preserves both `true` and explicit `false`. Member-shape
references retain caller-owned RDF identity; list length bounds retain exact
RDF values or independent compiled counts without a machine-integer limit.
Explicit `UniqueMembers(false)` relaxes uniqueness while still requiring a
valid SHACL list. `Debug` and `Trace` are messages that are not constraint
violations (`Severity::is_violation` returns `Some(false)`). `Closedness` keeps
`sh:closed true`, explicit `false` and `sh:ByTypes`. `NamedOrList` keeps whether
`sh:rootClass` or `sh:uniqueValuesFor` was written as one IRI or as a SHACL list.
An absent `sh:reificationRequired` is `None`, distinct from explicit `false`.
Consumers own parameter checking, list traversal, reifier lookup, type-based
property collection, target-node computation and report construction. These
declarations do not traverse or collect lists or graphs.

## Example

```rust
use iri_rfc3987::Iri;
use rdf_structural_model::{BlankNodeId, Literal};
use shacl_structural_model::{Component, DefaultTerm, NodeShape, PropertyShape, ShaclPath, Target};

let person: Iri<String> = "https://example.org/Person".parse().expect("valid IRI");
let name: Iri<String> = "https://example.org/name".parse().expect("valid IRI");

let name_shape = PropertyShape::<DefaultTerm>::new(
    DefaultTerm::BlankNode(BlankNodeId::try_new("urn:shapes:source".parse().unwrap(), "nameShape".to_owned()).unwrap()),
    ShaclPath::Predicate(name),
)
.with_component(Component::MinCount(DefaultTerm::literal(
    Literal::typed("1", "http://www.w3.org/2001/XMLSchema#integer".parse().unwrap()).unwrap(),
)));

let person_shape = NodeShape::<DefaultTerm>::new(
    DefaultTerm::iri("https://example.org/PersonShape".parse().expect("valid IRI")),
)
.with_target(Target::Class(person))
.with_property_shape(name_shape.id.clone());

assert_eq!(person_shape.property_count(), 1);
assert_eq!(name_shape.components[0].kind_name(), "minCount");
```

## Value types

RDF-valued fields default to `rdf_structural_model::Term`. Applications can instead use
`oxrdf::Term`, references to their existing terms, or opaque term handles.
Construction has no `Clone`, `Debug`, or Serde bounds; those traits are required
only when calling the corresponding operations. IRIs in IRI-only roles, such
as predicates and datatypes, default to `iri_rfc3987::Iri<String>`. The independent
`Named` parameter accepts a caller-owned named-node type when programmatic RDF
terms preserve relative or otherwise non-RFC lexical values. This does not
relax the default IRI parser or accept arbitrary RDF values in named roles.

```rust
use shacl_structural_model::Component;

let constraint: Component<String> = Component::Pattern {
    pattern: "^[A-Z]".to_owned(),
    flags: Some("i".to_owned()),
};

assert_eq!(constraint.kind_name(), "pattern");
```

Count and length parameters preserve the caller's RDF value type, including
signed and arbitrary-size integers. `Component<Value, Count>`,
`NodeShape<Value, Count>`, `PropertyShape<Value, Count>`, and parameterized
declarations also accept a separate compiled count type; `Count` defaults to
`Value`. The model does not parse lexical forms, impose a `u64` limit, or read
literal payloads. Order values stay as RDF terms, independently of `Count`.

Regex flag parameters likewise retain `Value` by default. The independent
third generic `Flags` is available on components, shapes, parameters, custom
components, target types and functions. An engine can retain an opaque RDF
reference beside its bounded compiled flags without materializing the original
string. Pattern parameters also retain `Value` by default, with an independent
fourth generic `Pattern` for compiled representations. For example,
`Component<Value, Count, Flags, Pattern>` allows four different types. Both
representations propagate through all declarations containing constraints.
The model does not parse regex syntax, compile patterns, or read payloads.
Consumers own compilation, resource accounting, and validation of RDF metadata.

Named roles use the fifth generic on components, shapes, parameters, custom
components, target types and functions. Paths use `ShaclPath<Named>`, and targets,
expressions, rules and parameter values propagate the same named type separately
from `Value`. Construction does not require conversion, cloning, display or
Serde traits. Display alone requires `Named: AsRef<str>`. Serialization and
checked loading use the selected named type's Serde contract; the strict
`Iri<String>` default continues to reject relative and invalid IRI strings.
Severity retains its independent checked IRI contract.

## Checked values, errors and ordering

`sh:languageIn` members are `LanguageTag` values. `LanguageTag::try_new`
checks BCP 47 well-formedness (without consulting the IANA registry), and Serde
decoding applies the same check, so a malformed tag cannot be loaded from JSON
or Postcard. Spelling is preserved; equality, hashing and ordering ignore ASCII
case.

```rust
use shacl_structural_model::{Component, LanguageTag, LexicalRole, StructuralModelError};

let languages = ["en", "nl-BE"].map(|tag| LanguageTag::try_new(tag).unwrap());
let constraint: Component<String> = Component::LanguageIn(languages.to_vec());
assert_eq!(constraint.display_value(), "en, nl-BE");
assert_eq!(
    LanguageTag::try_new("en_US"),
    Err(StructuralModelError::InvalidSyntax {
        lexical: "en_US".to_owned(),
        expected: LexicalRole::LanguageTag,
    })
);
```

`StructuralModelError` reports the offending lexical value with the
`LexicalRole` it was supplied for: `UnknownTerm` for a well-formed value outside
a closed vocabulary (an unknown `sh:nodeKind` IRI) and `InvalidSyntax` for a
malformed value (a severity that is not an absolute IRI, a malformed tag).

The structural value types implement `PartialOrd` and `Ord` whenever their
generic parameters do, so consumers can sort shapes, components, paths,
targets, expressions and rules canonically. The default
`rdf_structural_model::Term` has no order, so canonical sorting needs an
ordered caller term type. `Severity` orders from most to least severe:
`Violation < Warning < Info < Debug < Trace`, followed by application-defined severities in IRI
order. The `inference` declarations retain RDF literals and subjects, which are
unordered, and so remain unordered.

Every `sh:` IRI the model decodes or encodes is a documented constant in the
`vocabulary` module, such as `vocabulary::VIOLATION` and `vocabulary::IRI`.

## Serialization and RDF identity

The default terms use the shared `rdf-structural-model` foundation. Their
serialization supports JSON and non-self-describing binary formats such as
Postcard, including nested shapes, numeric lexical forms and directional
language messages. Blank nodes carry an explicit caller-assigned scope IRI;
use the same scope for intentionally shared identities and distinct scopes for
independent imports. This changes the default term API and serialized shape.

Applications still supply their own `Value`, `Count` and `Flags` without a
conversion layer. Their Serde support is conditional on those types. Model
support for RDF 1.2 triple terms and typed extensions does not establish SHACL
validator execution support for those features.

## Restricted SHACL 1.2 inference declarations

The `inference` module describes a model-only subset pinned to the
[28 October 2026 Inference Rules draft](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261028/)
and [Node Expressions draft](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261028/).
It is distinct from the AF declarations below. It does not execute rules or
certify inference-engine support.

| API | Representation |
| --- | --- |
| `RuleSet`, `RuleSetMembership`, `RuleSetInclusion` | Independent checked IRI declaration and edge records, suitable for streaming |
| `ResourceRef`, `ShapeRuleLink` | Shared RDF `Subject<Text>` identities, including scoped blank nodes |
| `RuleNumber`, `RuleSchedule` | Checked exact integer/decimal metadata; absent layer/order/runOnce retained |
| `TripleRuleDeclaration` | Independent rule identity, schedule, and three optional expression slots |
| `DirectExpression` | Checked direct IRI/literal/triple-term constants or `shnex:pathValues` with shared paths |

The new types accept shared RDF owned or borrowed text. Numeric `Eq`/hash retain
literal identity; `cmp_value` compares arbitrary-width exact values without
floating-point conversion, digit-buffer allocation or a general datatype engine.
Constructors and nested JSON/Postcard decoding apply the same semantic checks.

Rule-set includes remain edges; the consumer resolves references, cycles and
membership without hydrating the relation. A rule's global status requires the
relevant shapes graph's associations, not merely the selected rule set.

Bare blank nodes are rejected as direct constants. Explicit `shnex:focusNode`
expressions are rejected in this subset, including during decoding. Paths retain
ordered structure and require at least two sequence/alternative members.
Expression construction caps structural depth at 64 and term/path nodes at 4096;
already allocated caller inputs remain the caller's responsibility. Default
expression decoding additionally caps wire depth at 128, deserializer calls at
32768 and total visited text at 1 MiB. `deserialize_with_limits` accepts tighter
caller budgets. Cap source bytes as well: a format decoder may allocate text
before delivering a Serde visitor. Bounds apply per expression, not per dataset.

Other 1.2 expression forms, explicit focus expressions, rule-type catalogs,
conditions, deactivation, expected-derived predicates, processors, entailment
selection, SPARQL/template declarations and temporary/provenance triple policies
are not yet modelled; they belong to the SHACL 1.2 target and are added as
model-only declarations. Existing AF declarations retain their own contract.
Reference resolution, default application, scheduling, target/condition
validation, graph traversal, SPARQL execution and inference remain consumer-owned.

## SPARQL and Advanced Features

| Declaration | Structural types |
| --- | --- |
| SELECT constraints, ASK/SELECT validators, result annotations | `SparqlConstraint`, `SparqlValidator`, `ResultAnnotation` |
| Query literals, prefix resources and imports | `SparqlQuery`, `PrefixDeclaration`, `PrefixDeclarations` |
| Custom components and parameter values | `CustomConstraintComponent`, `CustomConstraint`, `Parameter`, `ParameterValues` |
| Direct SPARQL targets and reusable target types | `SparqlTarget`, `SparqlTargetType`, `CustomTarget` |
| ASK/SELECT functions | `ShaclFunction`, `SparqlSelectOrAsk` |
| All seven AF node-expression forms, including nested path inputs | `NodeExpression`, `ExpressionKind`, `ExpressionConstraint` |
| Triple and SPARQL rules, conditions, order and deactivation | `ShaclRule`, `RuleKind`, `TripleRule` |

Declarations retain source nodes and RDF-valued messages. Parameter values
are kept in full before an engine constructs query bindings. Prefix resources
and imports are references, so shared declarations need not be copied into
every query. Property shapes support targets as well as node shapes.
Implicit class targets and custom target-type identifiers use the caller's
RDF value type, preserving blank-node identity as well as named resources.

```rust
use shacl_structural_model::{Component, DefaultTerm, NodeShape, SparqlConstraint, SparqlQuery};

use rdf_structural_model::{BlankNodeId, Literal};

let blank = |label: &str| DefaultTerm::BlankNode(
    BlankNodeId::try_new("urn:shapes:source".parse().unwrap(), label.to_owned()).unwrap()
);
let query = SparqlQuery::new(DefaultTerm::literal(Literal::string(
    "SELECT $this WHERE { $this ex:status ex:Invalid }",
)))
.with_prefixes(blank("prefixes"));
let shape = NodeShape::<DefaultTerm>::new(blank("shape"))
    .with_component(Component::Sparql(Box::new(
        SparqlConstraint::new(blank("constraint"), query),
    )));
assert_eq!(shape.components[0].kind_name(), "sparql");
```

## Scope and memory

The crate exposes individual shapes and nested structural values. It does not
provide a document-sized `Schema` collection, so a parser can emit shapes one
at a time and let the caller decide whether to validate, serialize, persist,
or retain them. Collections inside a single shape represent that shape's
direct structural values; they are not a replacement for streaming an entire
input document.

This is a parsed structural model, not an RDF graph archive. Paths retain
their structure but not all RDF-list cell identities. Boolean parameters and
`sh:languageIn` members are checked language tags. Regex patterns and flags retain
their RDF values or caller-defined compiled representations. Callers that need
exact source triples or graph context must retain those separately. `path_node`
can retain a path's source root for result reporting. Display helpers are
human-readable labels, not RDF or SPARQL serialization.

The parser/consumer checks RDF node kinds, cardinality, query syntax, shape
references and expression cycles. This crate does not inspect opaque `Value`
payloads or certify a declaration as well-formed. Serde rejects unrecognized
fields on shapes and the new declarations, rather than discarding them.

## Migrating from 6.4.7

This is a breaking API and Serde format change:

- `Component::Closed { is_closed: bool, .. }` becomes
  `Component::Closed { closedness: Closedness, .. }`. Map `true` to
  `Closedness::Closed` and `false` to `Closedness::Open`
  (`Closedness::from_boolean`); `sh:ByTypes` is `Closedness::ByTypes`.
- `Severity` gains `Debug` and `Trace` before `Custom`. `sh:Debug` and
  `sh:Trace` no longer decode as `Severity::Custom`, and the Postcard variant
  index of `Severity::Custom` changes, so stored Postcard severities must be
  re-encoded.
- `Component` gains `SubsetOf`, `SomeValue`, `ReifierShape`, `RootClass` and
  `UniqueValuesFor`; exhaustive matches need the new variants.

## Migrating from 6.4

This is a breaking API and Serde-validation change:

- `Component::LanguageIn` holds `Vec<LanguageTag>` instead of `Vec<String>`.
  Construct members with `LanguageTag::try_new`. The JSON and Postcard
  encodings are unchanged, but stored malformed tags no longer decode.
- `StructuralModelError::UnknownNodeKind { value }` becomes
  `UnknownTerm { lexical, expected: LexicalRole::NodeKindIri }`, and
  `InvalidSeverityIri { value }` becomes
  `InvalidSyntax { lexical, expected: LexicalRole::SeverityIri }`.
- `Component`, the shapes and the declarations that contain them now also
  implement `Eq` and `Hash` when their generic parameters do.

## Migrating from 2.x

`Target::ImplicitClass`, `SparqlTargetType::id` and `CustomTarget::target_type`
now hold `Value` rather than `Iri<String>`. Pass the corresponding RDF term,
including for named resources. `Target::iri()` and `NodeShape::target_class_iris()`
cover explicit IRI-valued targets; they do not inspect opaque implicit-class
terms. This is a breaking API and serialized-shape change for callers whose
RDF value representation differs from an IRI string.

## Migrating from 1.x

This is a breaking API and Serde format change:

- Shape IDs, shape references, `Target::Node`, and message builders now take
  `Value`. Construct an RDF IRI/blank-node/literal term for those fields.
- `PropertyShape::name` and `description` become `names` and `descriptions`:
  vectors of RDF literals. Their builders append values, retaining languages.
- Shape and rule `order` values use `Option<Value>`, replacing floating-point
  ordering on property shapes. Node shapes now also have `order`.
- `ShaclRule::Sparql` becomes `ShaclRule::new(id, RuleKind::Sparql(query))`.
  Query text is an RDF literal in `SparqlQuery`, with explicit prefix references.
- `Severity` supports custom IRIs and is no longer `Copy`. Its accessors borrow
  the value. Invalid text returns an error.
- Shape `local_name()` methods are removed because IDs need not be IRIs.
  Inspect the caller's term type and use `local_name` for actual IRIs.
  `Target::iri()` now returns `Option` for Core class/predicate targets.
- Exhaustive matches on `Component`, `Target`, and `Severity` need the new
  variants. Migrate stored Serde values explicitly; no legacy fields are ignored.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.
