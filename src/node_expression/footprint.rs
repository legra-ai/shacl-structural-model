//! Structural budgets, computed once per expression from its nested footprints.

use super::{ExpressionError, ExpressionForm};
use crate::ShaclPath;

/// The depth and node count of a checked expression tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct Footprint {
    depth: usize,
    nodes: usize,
}

impl Footprint {
    pub(super) const MAX_DEPTH: usize = 64;
    pub(super) const MAX_NODES: usize = 4096;
    pub(super) const LEAF: Self = Self { depth: 1, nodes: 1 };

    pub(super) const fn depth(self) -> usize {
        self.depth
    }

    pub(super) const fn nodes(self) -> usize {
        self.nodes
    }

    /// Check one form and combine its children's already checked footprints.
    ///
    /// Each child's footprint is final, so this is linear in the number of
    /// direct operands rather than in the size of the tree.
    pub(super) fn of<Value, Named>(
        form: &ExpressionForm<Value, Named>,
    ) -> Result<Self, ExpressionError> {
        let mut total = Self::LEAF;
        let mut result = Ok(());
        form.for_each_child(|child| {
            if result.is_ok() {
                result = total.absorb(child.footprint());
            }
        });
        result?;
        match form {
            ExpressionForm::PathValues { path, .. } => {
                let mut budget = Self::MAX_NODES.saturating_sub(total.nodes);
                let depth = check_path(path, 1, &mut budget)?;
                total.absorb(Self {
                    depth,
                    nodes: Self::MAX_NODES - total.nodes - budget,
                })?;
            }
            ExpressionForm::List(list) => total.absorb(Self {
                depth: 1,
                nodes: list.len(),
            })?,
            ExpressionForm::CustomNamed(arguments) if arguments.is_empty() => {
                return Err(ExpressionError::NoNamedArgument);
            }
            _ => {}
        }
        Ok(total)
    }

    /// Add a nested footprint one level below this expression.
    fn absorb(&mut self, child: Self) -> Result<(), ExpressionError> {
        let depth = self.depth.max(child.depth.saturating_add(1));
        let nodes = self.nodes.saturating_add(child.nodes);
        if depth > Self::MAX_DEPTH || nodes > Self::MAX_NODES {
            return Err(ExpressionError::BudgetExceeded);
        }
        *self = Self { depth, nodes };
        Ok(())
    }
}

/// Check path arity and spend one node per path step; return the path depth.
fn check_path<Named>(
    path: &ShaclPath<Named>,
    depth: usize,
    budget: &mut usize,
) -> Result<usize, ExpressionError> {
    if depth >= Footprint::MAX_DEPTH || *budget == 0 {
        return Err(ExpressionError::BudgetExceeded);
    }
    *budget -= 1;
    match path {
        ShaclPath::Predicate(_) => Ok(depth),
        ShaclPath::Sequence(paths) | ShaclPath::Alternative(paths) => {
            if paths.len() < 2 {
                return Err(ExpressionError::InvalidPathArity);
            }
            paths.iter().try_fold(depth, |deepest, path| {
                Ok(deepest.max(check_path(path, depth + 1, budget)?))
            })
        }
        ShaclPath::Inverse(path)
        | ShaclPath::ZeroOrMore(path)
        | ShaclPath::OneOrMore(path)
        | ShaclPath::ZeroOrOne(path) => check_path(path, depth + 1, budget),
    }
}
