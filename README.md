# shacl-structural-model

[![crates.io](https://img.shields.io/crates/v/shacl-structural-model.svg)](https://crates.io/crates/shacl-structural-model)
[![Documentation](https://docs.rs/shacl-structural-model/badge.svg)](https://docs.rs/shacl-structural-model)
[![CI](https://github.com/legra-ai/shacl-structural-model/actions/workflows/ci.yml/badge.svg)](https://github.com/legra-ai/shacl-structural-model/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/shacl-structural-model.svg)](https://github.com/legra-ai/shacl-structural-model#license)

Typed structural values for SHACL shapes in Rust.

`shacl-structural-model` is a small model for representing SHACL shapes,
property paths, targets, constraint components, severities, messages, and
SHACL Advanced Features rule declarations. It is intended for parsers,
editors, validators, serializers, and storage adapters that need typed shape
values without taking a dependency on an RDF graph implementation.

The default value type is [`rdf_syntax::Term`]. The structural types are also
generic over the value type, so an application can use its own RDF term model
without copying or converting every constraint value.

This crate models structural values only. It does not parse RDF, read or
write files, access a graph store, execute validation, run SPARQL, or collect
an entire shapes graph. Callers should stream shapes from their input and
retain only the structural values needed by their operation.

## W3C specifications

The stable baseline is the [SHACL Recommendation](https://www.w3.org/TR/shacl/).
The model is also designed to track the [SHACL 1.2 Core Working Draft](https://www.w3.org/TR/shacl12-core/),
which is not yet a W3C Recommendation. SHACL-AF rule values are represented
without executing the rules; the relevant working-group material is described
in the [SHACL 1.2 Rules specification](https://www.w3.org/TR/shacl12-rules/).

## Example

```rust
use iri_rfc3987::Iri;
use shacl_structural_model::{Component, DefaultTerm, NodeShape, PropertyShape, ShaclPath, Target};

let person: Iri<String> = "https://example.org/Person".parse().expect("valid IRI");
let name: Iri<String> = "https://example.org/name".parse().expect("valid IRI");

let name_shape = PropertyShape::<DefaultTerm>::new(
    "https://example.org/nameShape".parse().expect("valid IRI"),
    ShaclPath::Predicate(name),
)
.with_component(Component::MinCount(1));

let person_shape = NodeShape::<DefaultTerm>::new(
    "https://example.org/PersonShape".parse().expect("valid IRI"),
)
.with_target(Target::Class(person))
.with_property_shape(name_shape.id.clone())
.with_component(Component::MinCount(1));

assert_eq!(person_shape.property_count(), 1);
assert_eq!(name_shape.components[0].kind_name(), "minCount");
```

## Value types

`Component`, `NodeShape`, and `PropertyShape` default to
`rdf_syntax::Term`, while remaining generic over a caller-provided value type:

```rust
use shacl_structural_model::Component;

let constraint: Component<String> = Component::Pattern {
    pattern: "^[A-Z]".to_owned(),
    flags: Some("i".to_owned()),
};

assert_eq!(constraint.kind_name(), "pattern");
```

## Scope and memory

The crate exposes individual shapes and nested structural values. It does not
provide a document-sized `Schema` collection, so a parser can emit shapes one
at a time and let the caller decide whether to validate, serialize, persist,
or retain them. Collections inside a single shape represent that shape's
direct structural values; they are not a replacement for streaming an entire
input document.

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT License

at your option.

Count and length parameters preserve the caller's RDF value type, including
signed and arbitrary-size integers. `Component<Value, Count>`,
`NodeShape<Value, Count>`, and `PropertyShape<Value, Count>` also accept a
separate compiled count type; `Count` defaults to `Value`. The model does not
parse lexical forms, impose a `u64` limit, or read literal payloads.
