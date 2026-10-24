# 0001: Preserve RDF terms at the structural/engine boundary

Status: Accepted

## Context

The 1.0 model restricted shape IDs and shape references to IRIs, target nodes
to IRIs, messages to Rust strings, and property ordering to `f64`. It could
not represent blank-node shapes, literal focus nodes, language-tagged messages,
or exact numeric order. Its only AF rule was an anonymous CONSTRUCT string.
These restrictions prevent its use as a shared parsed model by pyshacl-turbo.

Seven executable regressions reproduced these restrictions before changing
the model. Additional API tests required absent SPARQL/AF declaration types.

## Decision

Use the existing `Value` generic for shape identities and references, RDF
messages, annotations, query literals, parameters and numeric order. Keep the
independent `Count` generic introduced for arbitrary-size integer parameters.
IRI-only roles default to the checked IRI type. The independent `Named`
parameter represents caller-owned named-node domains separately from `Value`;
programmatic RDF values can retain lexical identities outside RFC IRI syntax.
The caller owns that named type's validation and checked Serde contract. The
default remains strict, and this adds no parser or unchecked IRI constructor. Constructors require no traits on
opaque values; serialization and cloning require their traits only when used.

Expose individual SPARQL constraints, validators, custom component definitions
and uses, prefix resources/import edges, target types, functions, node
expressions and rule declarations. References retain their source nodes;
recursive expression values retain their structure and argument order.
Preserve target and rule parameter values before query binding or evaluation.

The model owns no graph, document-wide schema, parser, I/O, query preparation,
validator or reasoner. Consumers stream declarations and choose their own
storage, validation and compilation strategy. Semantic checks stay with the
consumer, which can interpret its opaque terms. Serde rejects unknown fields
on shapes and the new declarations so unsupported data cannot silently vanish.

## Consequences

This requires a major version and an explicit migration of stored Serde data.
The shared release pipeline derives that version from a breaking conventional
commit; do not manually stamp a release before its tested revision exists.

Engine terms such as `oxrdf::Term`, borrowed terms and opaque handles can be
used directly. Oxrdf is a development-only dependency proving that boundary;
there is no new runtime dependency. This change does not install the model in
Turbo or establish pySHACL validation compatibility or a speed improvement.

This is not a lossless RDF graph archive. Original RDF-list cells, arbitrary
triples, graph names and interpreted boolean/string fields require separately
retained source data when an application needs them. Validation engines still
compile this model into their own indexes, regexes, queries and rule programs.

## References

- [SHACL](https://www.w3.org/TR/shacl/): shapes, messages, severity, SPARQL
  constraints, parameters, validators and prefix declarations.
- [SHACL Advanced Features](https://www.w3.org/TR/shacl-af/): targets, functions,
  expression constraints, nested path inputs, annotations and rules.
- `tests/rdf_identity.rs`, `tests/advanced_features.rs`, `tests/engine_terms.rs`.

## Caller-owned named roles

Programmatic RDF adapters can retain empty, relative and space-containing
named-node lexical values while still rejecting literal or blank values in a
named-only role. A strict absolute `Iri<String>` cannot represent that domain.
Propagate `Named` through predicate paths, class/datatype constraints, explicit
targets, parameter predicates, functions and custom components, including
nested expressions and rules. Keep existing `Value`, `Count`, `Flags` and
`Pattern` parameters independent. Default IRI loading remains checked, and
caller-specific named loading is checked by its selected type. Severity remains
an independent checked IRI type. There is no RDF-store runtime dependency,
normalization, alternate compiler, sentinel IRI or graph side table.
