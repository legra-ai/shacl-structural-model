# Pattern parameter representation

- Exclusive dependency lane: `/Users/jgeluk/Work/shacl-structural-model-memory`.
- Owner: Codex `/root`, assigned streaming literal lane; user approved patterns.
- Branch: `fix/regex-pattern-parameters`, based on published 4.0.0 `ae6771c`.
- Paired private lane: `/Users/jgeluk/Work/legra-memory`, `fix/llp05-schema-pattern`.
- The flags branch and its post-release handoff `7d1e4fb` are preserved.
- Two serialization regressions failed before the fix: opaque pattern values
  were rejected with `invalid type: map, expected a string`. RED log:
  `/tmp/shacl-pattern-model-red.log`.
- Fix: independent fourth generic Pattern, defaulting to Value, propagated
  through components, both shape kinds, parameters and all parameter owners.
  Callers may retain an RDF reference beside their compiled representation.
- Implementation commit `00fdf81` passes all nine focused tests and mandatory
  hooks. Both new test files are committed; no pending implementation changes.
  Coverage includes opaque RDF values, nested parameter constraints,
  independently compiled patterns/flags/counts, all declaration owners and
  construction without Clone/Debug/Serde bounds.
- Post-commit validation: all 32 library/integration tests, three doctests,
  five maintenance-policy tests and public documentation generation pass.
  The packaged archive includes every test and both licenses, and passes the
  same 32 tests plus three doctests using exact repository Rust 1.89.0.
  Logs: `/tmp/shacl-pattern-model-all.log`, `-doc.log`, `-rustdoc.log`, and
  `/tmp/shacl-pattern-model-package-pinned.log`. No new release exists yet.
- No regex parser/compiler or private storage implementation is included.
  This removes the structural String restriction only. Streaming source reads,
  compiler resource accounting and compiled-schema persistence remain private
  consumer work; do not report those as delivered by this model change.
- Existing main CI is the sole publisher. Do not bump locally or add a release
  path. Required checks/title/review settings remain unverified; this is not
  evidence of complete ADR 0105 adoption.

## Prior flag delivery

# Streaming schema flag parameters

- Exclusive dependency lane: `/Users/jgeluk/Work/shacl-structural-model-memory`.
- Owner: Codex `/root`, assigned streaming literal lane; user authorized flags.
- Branch: `fix/regex-flag-parameters`, based on published 3.0.0 at `2adc1d3`.
- Paired private lane: `/Users/jgeluk/Work/legra-memory`, `fix/llp05-schema-text`.
- The count branch and its post-release handoff `f628837` are preserved.
- Two public serialization regressions failed first: flag values and nested
  parameter constraints required a String and rejected opaque RDF values.
- Fix: independent Flags generic, defaulting to Value, propagated through all
  types containing parameter constraints. This supports retained RDF identity
  and bounded compiled flags without implementing graph/payload logic here.
- Pattern bodies remain strings; this change does not implement streamed regex
  compilation. No private payload implementation belongs in this public crate.
- Commit `f3b7e30` passes all three focused flag regressions and normal hooks.
  All 26 library/integration tests, three doctests, five maintenance-policy
  tests and documentation generation pass. The packaged archive passes the
  same 26 tests and three doctests. PR #8 is ready for review:
  https://github.com/legra-ai/shacl-structural-model/pull/8 . Merge watch active;
  registry publication is required before final private integration.
- Existing CI remains the sole publisher. Do not claim full ADR 0105 settings
  adoption; required checks/title/review configuration remains unverified.
