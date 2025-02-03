use std::sync::Weak;

use super::{dice::DEvalTree, stats::damage::Damage};

#[derive(Debug)]
pub struct Attack {
    roll: DEvalTree,
    damage: Damage,

    attacker: Weak<()>,
    via: Weak<()>,
    target: Weak<()>,
}

impl Attack {
    pub fn is_critical(&self) -> bool {
        is_critical(&self.roll)
    }
}

/// Is an attack critical?
fn is_critical(to_hit: &DEvalTree) -> bool {
    match to_hit {
        DEvalTree::Modifier(_) => false,
        DEvalTree::Roll(twenty) if twenty == &[20] => true,
        DEvalTree::Advantage(20, _) | DEvalTree::Advantage(_, 20) => true,
        DEvalTree::Disadvantage(20, 20) => true,
        DEvalTree::Add(t1, t2) => is_critical(t1) || is_critical(t2),
        DEvalTree::Sub(t1, t2) => is_critical(t1) || is_critical(t2),
        DEvalTree::Mul(t1, t2) => is_critical(t1) || is_critical(t2),
        DEvalTree::Div(t1, t2) => is_critical(t1) || is_critical(t2),
        _ => false,
    }
}

#[derive(Debug)]
pub struct Hit {
    attack: Attack,
}

impl Hit {
    pub fn is_critical(&self) -> bool {
        self.attack.is_critical()
    }
}
