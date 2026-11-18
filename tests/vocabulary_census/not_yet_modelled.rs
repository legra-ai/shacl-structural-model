//! `sh:` terms that the model does not represent yet.
//!
//! SHACL 1.2 is the target specification, so every entry here is a gap to
//! close, not an exclusion: either a term that belongs to the separate report
//! crate, or a term no SHACL 1.2 draft section defines yet. Only SHACL-JS is
//! outside the scope (ADR 0001).

const REPORT: &str = "belongs to report crate: validation reports, results and processor \
                      configurations are values a validator produces from a validation run, \
                      not shapes-graph declarations, so ADR 0001 places them outside this \
                      model-only shapes crate in a separate shacl-report-model crate";
const UPDATE: &str = "declared by the vocabularies without semantics in any SHACL 1.2 draft \
                      (Core, Node Expressions, SPARQL-Related Features, Inference Rules), so \
                      there is no dated section to model it from yet";

/// Each unmodelled term with the reason it is still missing.
pub(super) const NOT_YET_MODELLED: &[(&str, &str)] = &[
    ("AbstractResult", REPORT),
    ("ValidationReport", REPORT),
    ("ValidationResult", REPORT),
    ("conforms", REPORT),
    ("result", REPORT),
    ("focusNode", REPORT),
    ("resultPath", REPORT),
    ("resultSeverity", REPORT),
    ("resultMessage", REPORT),
    ("sourceConstraint", REPORT),
    ("sourceConstraintComponent", REPORT),
    ("sourceShape", REPORT),
    ("value", REPORT),
    ("detail", REPORT),
    ("ValidationActivity", REPORT),
    ("ValidationAgent", REPORT),
    ("usedDataGraph", REPORT),
    ("usedShapesGraph", REPORT),
    ("conformsToShapesGraph", REPORT),
    ("shapesGraphWellFormed", REPORT),
    ("ProcessorConfiguration", REPORT),
    ("conformanceDisallows", REPORT),
    ("update", UPDATE),
    ("SPARQLUpdateExecutable", UPDATE),
    ("describe", UPDATE),
    ("SPARQLDescribeExecutable", UPDATE),
];

/// AF node-expression terms the SHACL 1.2 vocabulary deprecates, each with the
/// 1.2 replacement that models it. ADR 0001 retains no superseded form.
pub(super) const SUPERSEDED: &[(&str, &str)] = &[
    ("this", "shnex:var \"focusNode\": ExpressionForm::Var"),
    (
        "filterShape",
        "shnex:filterShape: ExpressionForm::FilterShape",
    ),
    ("nodes", "shnex:nodes: ExpressionForm operands"),
    (
        "intersection",
        "shnex:intersection: ExpressionForm::Intersection",
    ),
    ("union", "shnex:concat: ExpressionForm::Concat"),
    ("minus", "shnex:remove: ExpressionForm::Remove"),
];

/// SHACL-JS terms, outside the scope by ADR 0001.
pub(super) const SHACL_JS: &[&str] = &[
    "js",
    "JSConstraint",
    "JSConstraint-js",
    "JSConstraintComponent",
    "JSExecutable",
    "JSFunction",
    "jsFunctionName",
    "jsLibrary",
    "JSLibrary",
    "jsLibraryURL",
    "JSRule",
    "JSTarget",
    "JSTargetType",
    "JSValidator",
];
