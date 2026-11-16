# Vendored W3C SHACL vocabularies

Development fixtures for `tests/vocabulary_census.rs`. They are not used at
runtime.

| File | Source | Retrieved | SHA-256 |
| --- | --- | --- | --- |
| `shacl.ttl` | <https://www.w3.org/ns/shacl.ttl> (SHACL 1.0 vocabulary, "Version from 2017-07-20"; served with `Last-Modified: Wed, 03 Sep 2025 18:39:04 GMT`) | 2026-11-16 | `0e5d8aea0eab98a072d4a02faaee1ee914ec99eab2ca473429726faed4a13f69` |
| `shacl12.ttl` | <https://raw.githubusercontent.com/w3c/data-shapes/a4a6301a149564f0fe208538c1d49f357c07dc84/shacl12-vocabularies/shacl.ttl> (SHACL 1.2 working-group vocabulary, commit `a4a6301` of 2026-11-11) | 2026-11-16 | `5a11ce0c0e7728dcb403f263b6ea938ef070828d9ff068b0ee714a5ae0cded3b` |

Both files are copied byte for byte. The 1.0 file is kept because it still
declares the SHACL-JS terms the 1.2 vocabulary drops; the census reads the
union of both. To track a newer draft, replace `shacl12.ttl` from the newest
`w3c/data-shapes` commit, update this table, and classify any new term.
