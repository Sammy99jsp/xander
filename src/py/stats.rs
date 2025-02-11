use std::{fmt::Display, fs::File, sync::Arc};

use pyo3::{exceptions::PyException, pyclass, pymethods, PyResult};

mod rs {
    pub(crate) use crate::core::{
        creature::Monster,
        dice::DExpr,
        stats::stat_block::{CreatureType, StatBlock},
    };
}

mod py {
    pub(crate) use crate::py::combat::action::*;
}

#[doc(hidden)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Stats(pub(in crate::py) Arc<rs::StatBlock>);

#[pymethods]
impl Stats {
    #[staticmethod]
    fn from_json(path: String) -> PyResult<Self> {
        let file = File::open(&path).map_err(PyException::new_err)?;
        let monster: rs::Monster =
            serde_json::from_reader(file).map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(Self(monster.0))
    }

    fn __repr__(&self) -> String {
        if self.0.is_dead() {
            return format!("{} <Dead>", self.0.name);
        }

        format!("{} <{}/{} HP>", self.0.name, self.0.hp(), self.0.max_hp())
    }

    fn _repr_html_(&self) -> String {
        fn divider() -> &'static str {
            r#"<svg height="5" width="100%" class="tapered-rule"><polyline points="0,0 400,2.5 0,5" fill="white"></polyline></svg>"#
        }

        #[rustfmt::skip]
        fn abilities(stats: &rs::StatBlock) -> String {
            fn modifier(mod_: &rs::DExpr) -> String {
                match mod_.evaluate().result() {
                    ..0 => mod_.to_string(),
                    0 => "±0".to_string(),
                    m @ 1.. => format!("+{m}")
                }
            }

            let str = (stats.scores.strength.get(), stats.modifiers.strength.get());
            let dex = (stats.scores.dexterity.get(), stats.modifiers.dexterity.get());
            let con = (stats.scores.constitution.get(), stats.modifiers.constitution.get());
            let int = (stats.scores.intelligence.get(), stats.modifiers.intelligence.get());
            let wis = (stats.scores.wisdom.get(), stats.modifiers.wisdom.get());
            let cha = (stats.scores.charisma.get(), stats.modifiers.charisma.get());
            format!(
            r#"<div style="display: grid; grid-template-columns: repeat(6, 1fr); grid-gap: 1em;">
                <div style="text-align: center;"><strong>STR</strong><br>{} ({})</div>
                <div style="text-align: center;"><strong>DEX</strong><br>{} ({})</div>
                <div style="text-align: center;"><strong>CON</strong><br>{} ({})</div>
                <div style="text-align: center;"><strong>INT</strong><br>{} ({})</div>
                <div style="text-align: center;"><strong>WIS</strong><br>{} ({})</div>
                <div style="text-align: center;"><strong>CHA</strong><br>{} ({})</div>
            </div>"#,
                str.0, modifier(&str.1),
                dex.0, modifier(&dex.1),
                con.0, modifier(&con.1),
                int.0, modifier(&int.1),
                wis.0, modifier(&wis.1),
                cha.0, modifier(&cha.1),
            )
        }

        fn title(title: &dyn Display) -> String {
            format!(
                r#"<h1 style="font-family: serif; margin-top: 0em; margin-bottom: 0em;">{title}</h1>"#
            )
        }

        fn field(name: &str, value: &dyn Display) -> String {
            format!("<span><strong>{name}</strong>&nbsp;{value}</span><br/>")
        }

        fn description(s: &rs::StatBlock) -> String {
            let size = &s.size;
            let info_str = match &s.ty {
                rs::CreatureType::Player => "".to_string(),
                rs::CreatureType::Monster(m) => {
                    format!(" {}, {}", m.ty, m.alignment)
                }
            };
            format!("<small><em>{size}{info_str}</em></small><br/>")
        }

        owo_colors::with_override(false, || {
            format!(
                r#"<div class="stat-block" style="padding: 1em; border: 1px solid white; width: fit-content;">
            {}{}{}{}{}{}{}{}
                </div>"#,
                title(&self.0.name),
                description(self.0.as_ref()),
                divider(),
                field("Armor Class", &self.0.ac.get().ac.result()),
                field("Hit Points", &self.0.max_hp()),
                field("Speed", &self.0.speeds),
                divider(),
                abilities(&self.0),
            )
        })
    }

    fn hp(&self) -> u32 {
        self.0.hp()
    }

    fn max_hp(&self) -> u32 {
        self.0.max_hp()
    }

    fn temp_hp(&self) -> Option<u32> {
        self.0.temp_hp()
    }

    fn actions(&self) -> Vec<py::Action> {
        self.0.actions.get().into_iter().map(py::Action).collect()
    }

    #[getter]
    fn dead(&self) -> bool {
        self.0.is_dead()
    }
}
