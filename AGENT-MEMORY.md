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
  daily. The previous tag-only publisher is removed. The first main release succeeded; required status/title/review gates in
  repository settings are still unverified/incomplete. Do not claim full
  ADR 0105 fleet/settings adoption.
- Backfilled already-published v0.1.0 at its recorded source `ede40e7`; this
  prevents the new pipeline from attempting to publish a different 0.1.0.
- PR https://github.com/legra-ai/shacl-structural-model/pull/4 merged on
  2026-10-11 at 11:37:34 UTC as `9bc89182c31c5ae5ffe84ef771d7c8a6779a7710`.
  The normal main pipeline published 1.0.0 from
  `316cccb30f7b004b2ff71bd32f7edb012eb15f24`; both main CI runs
  `38136251526` and `38136270948` passed. Public release:
  https://github.com/legra-ai/shacl-structural-model/releases/tag/v1.0.0.
  The consumed registry archive records that exact source revision.
- The old v0.1.0 tag backfill triggered its retired tag-only publisher, which
  refused the already-published version (run `38136048702`); no overwrite.
- The merge/release wait is complete. All seven private consumer regressions
  now pass against published 1.0.0; cold-reopen and wider integration checks
  continue in the paired Legra lane. No further upstream code is pending.
- No private Legra code or payload implementation belongs in this public crate.
