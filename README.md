# shacl-structural-model

[![crates.io](https://img.shields.io/crates/v/shacl-structural-model.svg)](https://crates.io/crates/shacl-structural-model)
[![Documentation](https://docs.rs/shacl-structural-model/badge.svg)](https://docs.rs/shacl-structural-model)
[![CI](https://github.com/legra-ai/shacl-structural-model/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/shacl-structural-model/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/shacl-structural-model.svg)](https://github.com/legra-ai/shacl-structural-model#license)

Typed structural values for SHACL shapes in Rust.

`shacl-structural-model` represents SHACL shapes, property paths, targets,
constraint components, severities, RDF messages, SPARQL declarations, SHACL 1.2
node expressions, SHACL 1.2 shape annotations, graph and entailment declarations,
SHACL 1.2 inference rules, and SHACL Advanced Features functions and rules. It is intended for parsers,
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
| `sh:targetNode` node expressions | `Target::Node(NodeExpression)` | [3.1.3.1](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#targetNode) |
| `sh:ShapeClass` implicit class targets | `Target::ImplicitClass { class, declaration: ClassDeclaration }` | [3.1.3.3](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#ShapeClass) |
| `sh:targetWhere` | `Target::Where(Value)` | [3.1.3.6](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#targetWhere) |
| `sh:shape` explicit shape targets | `ExplicitShapeTarget { node, shape }` (a data-graph record) | [3.1.3.7](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#x3-1-3-7-explicit-shape-targets-sh-shape) |
| `sh:values`, `sh:defaultValue` | `PropertyShape::values`, `PropertyShape::default_value` (`Option<NodeExpression>`) | [5](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#x5-node-expressions), [6.9.4.2](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#x6-9-4-2-value-nodes-of-property-shapes) |
| `sh:deactivated` node expressions | `NodeShape`/`PropertyShape::deactivated` (`Option<NodeExpression>`) | [3.1.6](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#deactivated) |
| `sh:deactivated` reifiers on constraint triples | `ConstraintDeactivation { shape, parameter, value, deactivated }` | [3.1.6](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#deactivated) |
| `sh:deactivated` on SPARQL-based constraints | `SparqlConstraint::deactivated` (`Option<NodeExpression>`) | SPARQL [3.2](https://www.w3.org/TR/2026/WD-shacl12-sparql-20261022/#sparql-constraints-syntax) |
| `sh:TripleTerm` node kind | `NodeKind::TripleTerm` | [6.10.1.3](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#NodeKindConstraintComponent) |
| `sh:intent`, `sh:formalized` | `ShapeAnnotations::intents` (`Intent { statement, formalized }`) | [6.11.2](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#intent) |
| `sh:agentInstruction` | `ShapeAnnotations::agent_instructions` | [6.11.3](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#agentInstruction) |
| `sh:codeIdentifier` | `ShapeAnnotations::code_identifier` (`CodeIdentifier`) | [6.11.4](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#codeIdentifier) |
| `sh:codeType` | `ShapeAnnotations::code_type` | [6.11.5](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#codeType) |
| `sh:unit` | `ShapeAnnotations::units` | [6.11.6](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#unit) |
| `sh:group`, `sh:PropertyGroup` | `PropertyShape::group`, `PropertyGroup { id, order }` | [6.11.8](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#group) |
| `sh:Graph`, `sh:DataGraph`, `sh:ShapesGraph` | `graph::GraphTypeDeclaration { graph, role: GraphRole }` | [6.1](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#shapes-graph), [6.2](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#data-graph) |
| `sh:shapesGraph`, `sh:dataGraph`, `sh:suggestedShapesGraph` | `graph::GraphLink { from, kind: GraphLinkKind, to }` | [6.5](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#sh-shapes-graph), [6.6](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#sh-data-graph) |
| `sh:entailment` | `graph::EntailmentDeclaration { shapes_graph, entailment: Entailment }` | [1.4](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#shacl-rdfs) |
| `sh:incomplete`, `sh:incompleteURIStart` | `graph::IncompleteNode`, `graph::IncompleteUriStart` (data-graph records) | [6.9.3.1](https://www.w3.org/TR/2026/WD-shacl12-core-20261116/#incomplete-data) |

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
declarations do not traverse or collect lists or graphs. A Core `sh:targetNode`
or `sh:defaultValue` constant is `NodeExpression::constant(term)`. `sh:shape`
triples live in the data graph, so `ExplicitShapeTarget` is a separate record
that a consumer streams beside its shapes.

`deactivated` is `None` when absent; a Core `sh:deactivated true` is
`NodeExpression::constant` of the boolean literal, and the consumer evaluates
the expression and checks that `true` or `false` is its only output node.
`ShapeAnnotations` sits on node and property shapes. Its literals keep their
language tags and directions, `sh:unit` may also be an IRI, and an absent
`sh:formalized` reifier is `None`, distinct from explicit `false`.
`CodeIdentifier` checks `^[a-zA-Z_][a-zA-Z0-9_]*$` on construction and
decoding. `PropertyGroup`, the `graph` records and `ConstraintDeactivation` are
separate triple-sized records a consumer streams beside its shapes; the model
resolves no graph link, import, entailment regime or incompleteness value.
Validation reports and results (Core 6.9) are values a validator produces,
so they belong to a separate report model rather than this shapes crate.

### SHACL 1.2 node expressions

`NodeExpression` follows [SHACL 1.2 Node Expressions, Working Draft 16 November 2026](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/)
and the SPARQL-based forms of [SHACL 1.2 SPARQL Extensions, Working Draft 22 October 2026](https://www.w3.org/TR/2026/WD-shacl12-sparql-20261022/).
`ExpressionForm` has one variant per node-expression function, and each
declaration cites its draft section:

| Function | Model | Draft section |
| --- | --- | --- |
| IRI, literal and triple-term constants | `ExpressionForm::Constant(Value)` | [3.1](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x3-1-constant-node-expressions) |
| `shnex:EmptyExpression` (`[]`) | `ExpressionForm::Empty` | [4.1.1](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#EmptyExpression) |
| `shnex:var` (`"focusNode"` replaces `sh:this`) | `ExpressionForm::Var(VariableName)` | [4.1.2](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-1-2-var-expressions) |
| List expressions | `ExpressionForm::List(ConstantList)` | [4.1.3](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#ListExpression) |
| `shnex:pathValues`, `shnex:focusNode` | `ExpressionForm::PathValues { path, path_node, focus_node }` | [4.1.4](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-1-4-path-values-expressions) |
| `shnex:exists` | `ExpressionForm::Exists` | [4.1.5](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#ExistsExpression) |
| `shnex:if`, `shnex:then`, `shnex:else` | `ExpressionForm::If { condition, branches: IfBranches }` | [4.1.6](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#IfExpression) |
| `shnex:distinct` | `ExpressionForm::Distinct` | [4.2.1](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#DistinctExpression) |
| `shnex:intersection` | `ExpressionForm::Intersection` | [4.2.2](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#IntersectionExpression) |
| `shnex:concat` (replaces `sh:union`) | `ExpressionForm::Concat` | [4.2.3](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#ConcatExpression) |
| `shnex:remove`, `shnex:nodes` (replaces `sh:minus`) | `ExpressionForm::Remove { remove, nodes }` | [4.2.4](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-2-4-remove-expressions) |
| `shnex:filterShape` | `ExpressionForm::FilterShape { shape, nodes }` | [4.2.5](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#FilterShapeExpression) |
| `shnex:limit`, `shnex:offset` | `ExpressionForm::Limit`, `ExpressionForm::Offset` | [4.2.6](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#LimitExpression), [4.2.7](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-2-7-offset-expressions) |
| `shnex:orderBy`, `shnex:desc` | `ExpressionForm::OrderBy { order_by, nodes, descending }` | [4.2.8](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-2-8-orderby-expressions) |
| `shnex:flatMap`, `shnex:findFirst`, `shnex:matchAll` | `ExpressionForm::FlatMap`, `FindFirst`, `MatchAll` (optional `nodes`) | [4.3](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-3-advanced-sequence-operations) |
| `shnex:count`, `shnex:min`, `shnex:max`, `shnex:sum` | `ExpressionForm::Aggregate { aggregate: Aggregate, input }` | [4.4](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-4-aggregation-expressions) |
| `shnex:instancesOf`, `shnex:nodesMatching` | `ExpressionForm::InstancesOf`, `ExpressionForm::NodesMatching` | [4.5.1](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#InstancesOfExpression), [4.5.2](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x4-5-2-nodes-matching-expressions) |
| `shnex:conformsToShape` | `ExpressionForm::ConformsToShape { node, shape }` | [4.5.3](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#ConformsToShapeExpression) |
| The 77 `sparql:<NAME>` functions | `ExpressionForm::Sparql { function: SparqlFunction, arguments }` | [5](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x5-sparql-functions) |
| `sh:select`, `sh:sparqlExpr`, `sh:prefixes` | `ExpressionForm::Select`, `ExpressionForm::SparqlExpr` (`SparqlExpressionText`) | SPARQL [6.1](https://www.w3.org/TR/2026/WD-shacl12-sparql-20261022/#x6-1-select-expressions), [6.2](https://www.w3.org/TR/2026/WD-shacl12-sparql-20261022/#x6-2-sparql-expr-expressions) |
| Custom named and list parameter expressions | `ExpressionForm::CustomNamed(Vec<NamedArgument>)`, `ExpressionForm::CustomList { function, arguments }` | [6.1](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x6-1-custom-named-parameter-functions), [6.2](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x6-2-custom-list-parameter-functions) |
| `shnex:arg` | `ExpressionForm::Arg(ArgumentKey)` | [6.3](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#ArgExpression) |
| `sh:NamedParameterExpressionFunction`, `sh:ListParameterExpressionFunction`, `sh:bodyExpression`, `sh:keyParameter` | `ExpressionFunction { id, style: ParameterStyle, parameters, body }`, `Parameter::key_parameter` | [6](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x6-custom-node-expressions) |
| `sh:expression` | `Component::Expression(ExpressionConstraint)` | [7.1](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#ExpressionConstraintComponent) |
| `sh:nodeByExpression` | `Component::NodeByExpression(NodeExpression)` | [7.2](https://www.w3.org/TR/2026/WD-shacl12-node-expr-20261116/#x7-2-sh-nodebyexpression) |

Built-in named parameter functions keep their parameters as typed fields.
List parameter functions (`sparql:` functions, `shnex:conformsToShape`, custom
list functions) keep positional argument lists; custom named parameter
expressions keep keyed `NamedArgument`s. A single list argument written without
its list is the same call and is stored as a one-member list.
`output_contract()` returns the `OutputContract` each draft section defines:
exactly one or at most one node, an ordered sequence, a distinct sequence, a
set in implementation-dependent order, or the contract of the selected branch,
function body or argument expression. The drafts define no unordered
multiset. `function_name()` returns the draft's function IRI.

Nested expressions are checked when built: sequence and alternative paths have
at least two members, `shnex:var` names are non-empty, a custom named parameter
expression has an argument, and `If` cannot omit both branches. A tree is capped
at depth 64 and at 4096 expression, path and list-member nodes; each node's
footprint is computed once from its children's. Decoding uses an unchecked wire
tree under `DecodeLimits` (by default wire depth 128, 32768 deserializer calls
and 1 MiB of visited text) and then applies the same checks bottom-up.
`NodeExpression::deserialize_with_limits` accepts tighter caller budgets. Cap
source bytes too: a format decoder may allocate text before a visitor sees it.
Bounds apply per expression, not per dataset. Constants and shape references
are caller-owned terms: the consumer checks that a constant is not a blank
node, that integers are valid, and that function arities match SPARQL.
Evaluation, scope handling, failure propagation and shapes-graph resolution
remain consumer-owned.

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
checks RFC 4647 basic language-range syntax, including `*`, rather than RFC 5646
language-tag syntax. Serde decoding applies the same check, so a malformed range cannot be loaded from JSON
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

## SHACL 1.2 inference declarations

The `inference` module describes the
[10 November 2026 Inference Rules draft](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/).
Its triple rules hold the full SHACL 1.2 node expressions above. It is
distinct from the AF declarations below. It does not execute rules or certify
inference-engine support.

| Declaration | Model | Draft section |
| --- | --- | --- |
| Rule types, `sh:SPARQLRule`, `sh:TripleRule` | `RuleType`, `RuleTyping { rule, rule_type }` | [3.1](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#rule-types) |
| `sh:rule` | `ShapeRuleLink` | [3.2](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#global-rules) |
| `sh:condition`, `sh:deactivated` | `RuleMetadata::conditions`, `RuleMetadata::deactivated` (`Option<bool>`) | [3.3](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#condition), [3.4](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#deactivated-rules) |
| `sh:layer`, `sh:order`, `sh:runOnce` | `RuleSchedule`, `RuleNumber` | [3.5](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#rule-layers), [3.6](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#rule-order), [3.7](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#run-once) |
| `sh:expectedPredicate` | `RuleMetadata::expected_predicates` | [3.8](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#expectedPredicate) |
| `sh:RuleSet`, `sh:hasRule`, `sh:includesRuleSet` | `RuleSet`, `RuleSetMembership`, `RuleSetInclusion` | [4](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#ruleSet) |
| `sh:RulesGraph` | `graph::GraphRole::RulesGraph` | [5](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#rules-graph) |
| `sh:RulesEntailment`, rule-set entailment | `graph::Entailment::Rules`, `graph::Entailment::Regime` | [6.1](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#RulesEntailment), [6.2](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#RuleSetEntailment) |
| `sh:tempTriple` | `InferredTripleReifier` with `InferenceAnnotation::TempTriple` | [7](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#tempTriple) |
| `sh:sourceRule` | `InferredTripleReifier` with `InferenceAnnotation::SourceRule` | [9](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#sourceRule) |
| `sh:ruleProcessor` | `RuleProcessor` on `RuleMetadata` and `RuleSet` | [10](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#ruleProcessor) |
| SPARQL rules: `sh:construct`, `sh:prefixes` | `SparqlRuleDeclaration`, `ConstructQuery` | [11.1](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#SPARQLRule) |
| Triple rules: `sh:subject`, `sh:predicate`, `sh:object` | `TripleRuleDeclaration`, `RuleExpression` | [11.2](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#TripleRule) |
| `sh:SPARQLRuleTemplate` and its instances | `SparqlRuleTemplate`, `TemplateParameter`, `TemplateRuleDeclaration` | [12](https://www.w3.org/TR/2026/WD-shacl12-inference-rules-20261110/#SPARQLRuleTemplate) |

Every rule declaration carries a `RuleMetadata` that retains absence: an
absent `sh:deactivated` or `sh:runOnce` is `None`, distinct from explicit
`false`. `ResourceRef` (`Subject<Text>`) identities include scoped blank nodes.
The new types accept shared RDF owned or borrowed text. Numeric `Eq`/hash retain
literal identity; `cmp_value` compares arbitrary-width exact values without
floating-point conversion, digit-buffer allocation or a general datatype engine.
`ConstructQuery` requires an `xsd:string` literal and does not parse it.
Constructors and nested JSON/Postcard decoding apply the same checks.

Rule-set includes, rule typings and reifier annotations remain individual
records; the consumer resolves references, cycles, membership, template
parameters and processor support without hydrating the relation. A rule's
global status requires the relevant shapes graph's associations, not merely
the selected rule set. Default application, scheduling, condition validation,
derived-triple computation, graph traversal, SPARQL execution and inference
remain consumer-owned. Existing AF declarations retain their own contract.

## SPARQL and Advanced Features

| Declaration | Structural types |
| --- | --- |
| SELECT constraints, ASK/SELECT validators, result annotations | `SparqlConstraint`, `SparqlValidator`, `ResultAnnotation` |
| Query literals, prefix resources and imports | `SparqlQuery`, `PrefixDeclaration`, `PrefixDeclarations` |
| Custom components and parameter values | `CustomConstraintComponent`, `CustomConstraint`, `Parameter`, `ParameterValues` |
| Direct SPARQL targets and reusable target types | `SparqlTarget`, `SparqlTargetType`, `CustomTarget` |
| ASK/SELECT functions | `ShaclFunction`, `SparqlSelectOrAsk` |
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
their structure but not all RDF-list cell identities. Boolean parameters are checked,
and `sh:languageIn` members are checked basic language ranges. Regex patterns and flags retain
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
- `Component` gains `SubsetOf`, `SomeValue`, `ReifierShape`, `RootClass`,
  `UniqueValuesFor` and `NodeByExpression`; exhaustive matches need the new
  variants.
- The AF `NodeExpression { node, kind: ExpressionKind }` is replaced by the
  SHACL 1.2 `NodeExpression`, built with `NodeExpression::try_new(ExpressionForm)`
  and an optional `with_source`. The 1.2 vocabulary deprecates the AF terms:
  `sh:this` becomes `NodeExpression::focus_node()`, `sh:path` becomes
  `ExpressionForm::PathValues`, `sh:filterShape` becomes `FilterShape`,
  `sh:union` becomes `Concat`, `sh:intersection` stays `Intersection`, and an AF
  function call becomes `CustomList` or `Sparql`. `ExpressionConstraint`,
  `TripleRule` and `Target::Node` hold the new type.
- `inference::DirectExpression` is replaced by `inference::RuleExpression`;
  `inference::DecodeLimits` moves to the crate root as `DecodeLimits`, and the
  expression variants of `InferenceModelError` become `ExpressionError`.
  Explicit `shnex:focusNode` inputs are now accepted. Blank-node constants are
  the consumer's check, as for every other caller-owned term.
- `Target::Node` holds a `NodeExpression`; `Target::ImplicitClass(Value)`
  becomes `Target::ImplicitClass { class, declaration }`; `Target` gains `Where`.
- `PropertyShape::default_value` holds a `NodeExpression`, and `PropertyShape`
  gains `values`. `Parameter` gains `key_parameter`.
- `NodeShape::deactivated`, `PropertyShape::deactivated` and
  `SparqlConstraint::deactivated` become `Option<NodeExpression>`: map `false`
  to `None` or an explicit constant, and `true` to
  `NodeExpression::constant` of `"true"^^xsd:boolean`. `with_deactivated`
  takes the expression. `SparqlConstraint` gains the `Named` generic.
- Node and property shapes gain `annotations: ShapeAnnotations`, and
  `PropertyShape` gains `group`. `NodeKind` gains `TripleTerm` as its last
  variant, so existing Postcard indexes are unchanged.
- `inference::TripleRuleDeclaration::with_schedule`/`schedule` become
  `with_metadata`/`metadata` over `RuleMetadata`, which holds the schedule.
  `RuleSet` gains `processors`. `INFERENCE_DRAFT` moves to the 10 November
  2026 draft, and `InferenceModelError` gains `InvalidConstructQuery`.

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
