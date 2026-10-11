# Streaming schema count parameters

- Exclusive dependency lane: `/Users/jgeluk/Work/shacl-structural-model-memory`.
- Owner: Codex `/root`, user-authorized numeric schema slice.
- Branch: `fix/integer-count-parameters`, base `49f237c`.
- Paired private lane: `/Users/jgeluk/Work/legra-memory`, `fix/llp05-schema-counts`.
- Two public serialization regressions failed before implementation: scalar
  and qualified count fields required u64 and rejected full integer values.
- Fix: independent Count generic on components and shapes, defaulting to Value.
  This preserves full values and supports bounded compiled counts downstream.
- Validation in progress. No PR yet. Normal CI/release migration under ADR 0105
  is required before consuming a published revision; existing CI is tag-only.
- No private Legra code or payload implementation belongs in this public crate.
