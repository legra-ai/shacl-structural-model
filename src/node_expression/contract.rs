//! The output-sequence contract each node-expression function guarantees.

/// The shape of a function's output nodes, as its draft section defines it.
///
/// This is the function's own guarantee, not an inference over its inputs:
/// for example `shnex:limit` keeps the order of its input list even when
/// that input is a path-values set in implementation-dependent order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OutputContract {
    /// Exactly one output node: a constant, a boolean or a count/sum.
    ExactlyOne,
    /// Zero or one output node: list parameter functions, var/arg values,
    /// `shnex:findFirst`, `shnex:min`/`shnex:max` and the empty expression.
    AtMostOne,
    /// An ordered list that may repeat nodes.
    OrderedSequence,
    /// An ordered list without repeated nodes (first occurrence kept).
    DistinctSequence,
    /// Distinct nodes in an implementation-dependent order.
    Set,
    /// The output of whichever if-expression branch is selected.
    SelectedBranch,
    /// The output of the custom function's `sh:bodyExpression`.
    FunctionBody,
    /// The output of the argument expression of a named parameter function.
    ArgumentExpression,
}

impl OutputContract {
    /// Whether the draft guarantees an order for the output nodes.
    ///
    /// Returns `None` when the contract defers to another expression.
    #[must_use]
    pub const fn is_ordered(self) -> Option<bool> {
        match self {
            Self::ExactlyOne | Self::AtMostOne | Self::OrderedSequence | Self::DistinctSequence => {
                Some(true)
            }
            Self::Set => Some(false),
            Self::SelectedBranch | Self::FunctionBody | Self::ArgumentExpression => None,
        }
    }

    /// Whether the draft guarantees the output has no repeated nodes.
    ///
    /// Returns `None` when the contract defers to another expression.
    #[must_use]
    pub const fn is_distinct(self) -> Option<bool> {
        match self {
            Self::ExactlyOne | Self::AtMostOne | Self::DistinctSequence | Self::Set => Some(true),
            Self::OrderedSequence => Some(false),
            Self::SelectedBranch | Self::FunctionBody | Self::ArgumentExpression => None,
        }
    }
}
