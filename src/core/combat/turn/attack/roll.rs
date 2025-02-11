//! Attack Rolls

use crate::{core::dice::DEvalTree, vis::rich::RichFormatting};

#[derive(Debug, Clone, PartialEq)]
#[repr(transparent)]
pub struct AttackRoll(pub(crate) DEvalTree);

impl std::fmt::Display for AttackRoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.criticality() {
            Some(Criticality::Success) => write!(f, "Critical(Success, {})", self.0),
            Some(Criticality::Failure) => write!(f, "Critical(Failure, {})", self.0),
            None => self.0.fmt(f),
        }
    }
}

impl RichFormatting for AttackRoll {
    fn html(&self) -> String {
        owo_colors::with_override(false, || {
            format!(
                "<code>{}</code>{}",
                self.0,
                match self.criticality() {
                    Some(_) => " &mdash; Critical",
                    None => "",
                }
            )
        })
    }
}

impl AttackRoll {
    pub fn criticality(&self) -> Option<Criticality> {
        is_critical(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Criticality {
    /// Critical success (20)
    Success,
    /// Critical failure (1)
    Failure,
}

/// Is an attack critical?
fn is_critical(to_hit: &DEvalTree) -> Option<Criticality> {
    use Criticality::*;
    match to_hit {
        DEvalTree::Roll(twenty) if twenty == &[20] => Some(Success),
        DEvalTree::Roll(one) if one == &[1] => Some(Failure),

        DEvalTree::Advantage(20, _) | DEvalTree::Advantage(_, 20) => Some(Success),
        DEvalTree::Advantage(1, 1) => Some(Failure),

        DEvalTree::Disadvantage(20, 20) => Some(Success),
        DEvalTree::Disadvantage(_, 1) | DEvalTree::Advantage(1, _) => Some(Failure),

        DEvalTree::Add(t1, t2) => is_critical(t1).or(is_critical(t2)),
        DEvalTree::Sub(t1, t2) => is_critical(t1).or(is_critical(t2)),
        DEvalTree::Mul(t1, t2) => is_critical(t1).or(is_critical(t2)),
        DEvalTree::Div(t1, t2) => is_critical(t1).or(is_critical(t2)),
        _ => None,
    }
}
