use std::sync::Arc;

use pyo3::{pyclass, pymethods, PyResult};

mod py {
    pub(crate) use crate::py::{
        combat::{
            attack::{Attack, AttackResult},
            speed::{SpeedType, WALKING},
        },
        legality::Legality,
    };
}

mod rs {
    pub(crate) use crate::{
        core::{combat::turn::TurnCtx, geom::P3},
        utils::legality::Legality,
    };
}

#[pyclass]
pub struct Turn(pub(super) Arc<rs::TurnCtx>);

#[pymethods]
impl Turn {
    #[pyo3(name = "move")]
    #[pyo3(signature = (delta, mode = py::WALKING))]
    fn try_move(&self, delta: (f32, f32, f32), mode: py::SpeedType) -> PyResult<py::Legality> {
        let (x, y, z) = delta;
        self.0
            .movement
            .try_move(mode.0, rs::P3::new(x, y, z))
            .try_into()
    }

    fn attack(&self, attack: py::Attack, target: (f32, f32, f32)) -> PyResult<py::Legality> {
        let me = self.0.combatant().upgrade().unwrap();
        let (x, y, z) = target;

        // Check if we've already used all actions for this turn, and return early if so.
        // If not, update the used action count by 1, and continue.
        if let l @ rs::Legality::Illegal(_) = self.0.actions.can_use() {
            return l.try_into();
        }

        attack
            .0
            .make_attack(&me, rs::P3::new(x, y, z))
            .map(py::AttackResult)
            .map(|res| {
                self.0.actions.mark_used();
                res
            })
            .try_into()
    }

    fn end(&self) -> PyResult<py::Legality> {
        let me = self.0.combatant().upgrade().unwrap();
        let combat = me.combat.upgrade().unwrap();
        combat.initiative.advance_turn();

        py::Legality::void_success()
    }

    #[pyo3(signature = (mode = py::WALKING))]
    fn possible_directions(&self, mode: py::SpeedType) -> PyResult<py::Legality> {
        // Is there any movement left? If not, return early, stating it's illegal.
        if let l @ rs::Legality::Illegal(_) = self.0.movement.any_movement_left(mode.0) {
            return l.try_into();
        }

        let me = self.0.combatant().upgrade().unwrap();
        let combat = me.combat.upgrade().unwrap();

        const DIRECTIONS: [(f32, f32, f32); 8] = [
            (0.0, 5.0, 0.0),
            (5.0, 5.0, 0.0),
            (5.0, 0.0, 0.0),
            (5.0, -5.0, 0.0),
            (0.0, -5.0, 0.0),
            (-5.0, -5.0, 0.0),
            (-5.0, 0.0, 0.0),
            (-5.0, 5.0, 0.0),
        ];

        rs::Legality::Legal(
            DIRECTIONS
                .map(|(x, y, z)| rs::P3::new(x, y, z))
                .into_iter()
                .filter_map(|p| {
                    combat
                        .arena
                        .is_passable(
                            rs::P3::from(me.position.get_cloned().unwrap().coords + p.coords),
                            me.stats.size,
                        )
                        .is_legal()
                        .then_some((p.x, p.y, p.z))
                })
                .collect::<Vec<_>>(),
        )
        .try_into()
    }

    fn __repr__(&self) -> String {
        format!(
            "Turn(movement_used = {}, actions_used = {})",
            self.0.movement.used(),
            self.0.actions.used()
        )
    }
}
