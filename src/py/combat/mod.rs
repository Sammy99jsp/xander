use std::{
    collections::HashMap,
    fmt::Write,
    sync::{Arc, RwLock},
};

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    pyclass, pymethods, PyObject, PyResult, Python,
};

pub mod action;
pub mod arena;
pub mod attack;
pub mod speed;
pub mod turn;

mod rs {
    pub(crate) use crate::core::{
        combat::{arena, Combat, Combatant, InitiativeRoll},
        geom::P3,
    };
}

#[derive(Debug, Default)]
pub struct Hooks {
    combatants: HashMap<usize, PyObject>,
}

#[pyclass]
pub struct Combat {
    combat: Arc<rs::Combat>,
    hooks: Hooks,
}

#[pymethods]
impl Combat {
    #[new]
    fn __init__(arena: PyObject) -> PyResult<Self> {
        Python::with_gil(|py| {
            if let Ok(arena::Simple(simple)) = arena.extract::<arena::Simple>(py) {
                return Ok(Self {
                    combat: rs::Combat::new(None, |weak| rs::arena::SimpleArena::new(weak, simple)),
                    hooks: Hooks::default(),
                });
            }

            Err(PyTypeError::new_err("Expected an Arena here"))
        })
    }

    #[getter]
    fn arena(&self) -> arena::Arena {
        arena::Arena(Arc::downgrade(&self.combat.arena))
    }

    fn join(
        &mut self,
        monster: super::Stats,
        name: String,
        position: (f32, f32, f32),
    ) -> Combatant {
        let (x, y, z) = position;
        let combatant = Arc::new(rs::Combatant {
            combat: Arc::downgrade(&self.combat),
            name,
            initiative: rs::InitiativeRoll(monster.0.initiative.get().result()),
            stats: monster.0,
            position: RwLock::new(rs::P3::new(x, y, z)),
        });

        self.combat.initiative.add(combatant.clone());
        Combatant(combatant)
    }

    fn _repr_html_(&self) -> String {
        let mut s = r#"<div style="display: grid; grid-template-columns: max-content max-content max-content max-content; gap: 4px 1em; margin: 0 25vw;">
        <span style="font-weight: bold;"></span>
        <span style="font-weight: bold;">Initiative</span>
        <span style="font-weight: bold;">Name</span>
        <span style="font-weight: bold;">Health</span>"#.to_string();

        let current = self.combat.initiative.current();
        for combatant in self.combat.initiative.as_vec() {
            write!(
                s,
                r#"<span style="font-weight: bold;">{}</span>
                <span>{}</span>
                <span>{}</span>
                <span>{} / {} HP</span>"#,
                if Arc::ptr_eq(&combatant, &current) {
                    "★"
                } else if combatant.stats.is_dead() {
                    "💀"
                } else {
                    ""
                },
                combatant.initiative.0,
                &combatant.name,
                combatant.stats.hp(),
                combatant.stats.max_hp()
            )
            .unwrap();
        }

        s.write_str("</div>").unwrap();
        s
    }

    fn __repr__(&self) -> String {
        format!("Combat({} members)", self.combat.len())
    }

    fn __len__(&self) -> usize {
        self.combat.len()
    }

    fn set_combatant_hook(&mut self, combatant: Combatant, hook: PyObject) -> PyResult<()> {
        self.hooks
            .combatants
            .insert(Arc::as_ptr(&combatant.0) as usize, hook);

        Ok(())
    }

    fn step(&mut self) -> PyResult<PyObject> {
        // Update internal state.
        self.combat.step();

        let current = self.combat.initiative.current();

        // Get the appropriate hook.
        let hook = self
            .hooks
            .combatants
            .get(&(Arc::as_ptr(&current) as usize))
            .ok_or_else(|| {
                PyValueError::new_err(format!("Expected a hook registered for {}", current.name))
            })?;

        // Called after .step(), so .current_turn() should not be None !
        let turn = self.combat.initiative.current_turn().unwrap();

        // Attempt to call it.
        Python::with_gil(|py| hook.call1(py, (turn::Turn(turn),)))
    }

    #[getter]
    fn current(&self) -> Combatant {
        Combatant(self.combat.initiative.current().clone())
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct Combatant(Arc<rs::Combatant>);

#[pymethods]
impl Combatant {
    #[getter]
    fn name(&self) -> String {
        self.0.name.clone()
    }

    #[getter]
    fn position(&self) -> (f32, f32, f32) {
        let p = self.0.position.read().expect("Not poisoned!");
        (p.x, p.y, p.z)
    }

    #[getter]
    fn stats(&self) -> super::Stats {
        super::Stats(self.0.stats.clone())
    }

    fn __repr__(&self) -> String {
        format!(
            "<{} {}/{} HP>",
            self.0.name,
            self.0.stats.hp(),
            self.0.stats.max_hp()
        )
    }

    fn __eq__(&self, other: &Combatant) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
