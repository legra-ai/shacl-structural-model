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
  same 26 tests and three doctests. PR #8 merged at 2026-10-11 15:13:36 UTC
  as `7a77c60ebb1752e87f281edf0971c6cffc76f2f2`; merge watch complete.
- Main CI run 38150389781 passed and published 4.0.0 from
  `ae6771ca9479278e9e4a4486fc809d456e23cc42` at 15:15:31 UTC.
  https://github.com/legra-ai/shacl-structural-model/releases/tag/v4.0.0
  The private lane is consuming this registry release and validating its reader.
- Existing CI remains the sole publisher. Do not claim full ADR 0105 settings
  adoption; required checks/title/review configuration remains unverified.
