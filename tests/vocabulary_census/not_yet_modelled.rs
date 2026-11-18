//! `sh:` terms that the model does not represent yet.
//!
//! SHACL 1.2 is the target specification, so every entry here is a gap to
//! close, not an exclusion. Only SHACL-JS is outside the scope (ADR 0001).

const REPORT: &str = "validation reports and results have no structural value yet";
const GRAPHS: &str = "graph-level and processor declarations have no structural value yet";
const GROUPS: &str = "property groups are not retained on property shapes yet";
const UPDATE: &str = "SPARQL UPDATE/DESCRIBE executables are not modelled yet";
const CORE12: &str = "SHACL 1.2 Core draft declaration not modelled yet";
const RULES12: &str = "SHACL 1.2 Inference Rules vocabulary not modelled yet";

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
    ("incomplete", REPORT),
    ("ValidationActivity", REPORT),
    ("ValidationAgent", REPORT),
    ("usedDataGraph", REPORT),
    ("usedShapesGraph", REPORT),
    ("conformsToShapesGraph", REPORT),
    ("shapesGraph", GRAPHS),
    ("shapesGraphWellFormed", GRAPHS),
    ("suggestedShapesGraph", GRAPHS),
    ("entailment", GRAPHS),
    ("dataGraph", GRAPHS),
    ("DataGraph", GRAPHS),
    ("Graph", GRAPHS),
    ("ShapesGraph", GRAPHS),
    ("ProcessorConfiguration", GRAPHS),
    ("conformanceDisallows", GRAPHS),
    ("incompleteURIStart", GRAPHS),
    ("group", GROUPS),
    ("PropertyGroup", GROUPS),
    ("update", UPDATE),
    ("SPARQLUpdateExecutable", UPDATE),
    ("describe", UPDATE),
    ("SPARQLDescribeExecutable", UPDATE),
    ("agentInstruction", CORE12),
    ("codeIdentifier", CORE12),
    ("codeType", CORE12),
    ("expectedPredicate", CORE12),
    ("formalized", CORE12),
    ("intent", CORE12),
    ("TripleTerm", CORE12),
    ("unit", CORE12),
    ("ruleProcessor", RULES12),
    ("RulesEntailment", RULES12),
    ("RulesGraph", RULES12),
    ("sourceRule", RULES12),
    ("SPARQLRuleTemplate", RULES12),
    ("tempTriple", RULES12),
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
