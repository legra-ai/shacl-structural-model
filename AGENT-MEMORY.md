# Streaming schema count parameters

- Exclusive dependency lane: `/Users/jgeluk/Work/shacl-structural-model-memory`.
- Owner: Codex `/root`, user-authorized numeric schema slice.
- Branch: `fix/integer-count-parameters`, base `49f237c`.
- Paired private lane: `/Users/jgeluk/Work/legra-memory`, `fix/llp05-schema-counts`.
- Two public serialization regressions failed before implementation: scalar
  and qualified count fields required u64 and rejected full integer values.
- Fix: independent Count generic on components and shapes, defaulting to Value.
  This preserves full values and supports bounded compiled counts downstream.
- Validation: all three focused tests, three library tests, two doctests,
  Clippy, and docs pass. The same six tests and two doctests pass from the
  packaged archive. Two policy regressions failed before pipeline adoption
  and now pass. Normal commit hooks passed for `bcafa99`.
- CI now uses the shared prepare/test/publish pipeline at one tested revision,
  tracks Cargo.lock, enforces scoped commit titles, and checks dependencies
  daily. The previous tag-only publisher is removed. Full settings/release
  evidence is pending the first main release; do not claim complete adoption.
- Backfilled already-published v0.1.0 at its recorded source `ede40e7`; this
  prevents the new pipeline from attempting to publish a different 0.1.0.
- No PR yet. Legra has seven reproduced failing count/length regressions.
  Registry publication is required before final private integration.
- No private Legra code or payload implementation belongs in this public crate.
