# Release approval

Every major version increase requires the user's explicit approval of the exact
version and release scope before changing that number or publishing it.
General permission to implement, merge, maintain, or release does not suffice.
Preserve accurate breaking-change markers and stop at the major-release gate.
Never approve an environment through owner credentials without that specific
user authorization. Patch and minor releases retain the existing CI contract.

## Zero-user pre-production versions

This repository has zero users. Fixes use patch releases; features and breaking
changes use minor releases. Preserve accurate breaking-change markers and use
`breaking-change-bump: minor` for both planning and preparation. A major increase
still requires the user's specific explicit approval. Do not add migrations or
compatibility paths for hypothetical consumers.
