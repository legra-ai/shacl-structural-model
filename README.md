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
pySHACL compatibility. SHACL-JS and complete SHACL 1.2 draft coverage are outside
the current scope.

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
as predicates and datatypes, continue to use `iri_rfc3987::Iri<String>`.

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
string. The model does not interpret flag syntax. Pattern bodies remain strings.

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
their structure but not all RDF-list cell identities. Boolean parameters and Core pattern/language strings are interpreted fields.
Regex flags retain their RDF values or a caller-defined compiled representation. Callers that need exact
source triples or graph context must retain those separately. `path_node`
can retain a path's source root for result reporting. Display helpers are
human-readable labels, not RDF or SPARQL serialization.

The parser/consumer checks RDF node kinds, cardinality, query syntax, shape
references and expression cycles. This crate does not inspect opaque `Value`
payloads or certify a declaration as well-formed. Serde rejects unrecognized
fields on shapes and the new declarations, rather than discarding them.

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
  the value. Invalid text returns `InvalidSeverityIri`.
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
