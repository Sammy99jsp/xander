#![feature(
    rustc_attrs,
    int_roundings,
    ptr_as_ref_unchecked,
    const_option,
    box_patterns,
    let_chains,
    ptr_fn_addr_eq,
    assert_matches,
    fn_traits,
    trait_alias,
    debug_closure_helpers,
    iter_intersperse,
    get_mut_unchecked,
    cell_update,
)]
#![allow(internal_features)]

use pyo3::prelude::*;
pub mod core;
pub mod utils;

#[doc(hidden)]
pub(crate) mod py;
pub mod serde;

#[pymodule]
mod xander {
    use pyo3::pymodule;

    #[pymodule]
    mod engine {
        use pyo3::prelude::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("xander.engine", m)
            })
        }

        #[pymodule]
        mod dice {
            use pyo3::prelude::*;

            use crate::{core::dice::*, py};

            #[pymodule_init]
            fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
                Python::with_gil(|py| {
                    py.import("sys")?
                        .getattr("modules")?
                        .set_item("xander.engine.dice", m)
                })?;

                m.add_class::<Die>()?;
                m.add_class::<py::DExpr>()?;
                m.add_class::<py::DEvalTree>()?;

                // TODO: Move this out!
                m.add("D4", D4)?;
                m.add("D6", D6)?;
                m.add("D8", D8)?;
                m.add("D10", D10)?;
                m.add("D12", D12)?;
                m.add("D20", D20)?;
                m.add("D100", D100)?;

                m.add_function(wrap_pyfunction!(crate::core::dice::set_seed, m)?)?;
                m.add_function(wrap_pyfunction!(crate::core::dice::random_seed, m)?)?;

                Ok(())
            }
        }
    }
}
// fn xander(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(double, m)?)?;

//     let engine = PyModule::new(m.py(), "engine")?;
//     m.add_submodule(&engine)?;
//     let dice = PyModule::new(engine.py(), "dice")?;

//     dice.add_class::<Die>()?;
//     engine.add_submodule(&dice)?;

//     Ok(())
// }
