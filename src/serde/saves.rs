use std::num::NonZeroU32;

use serde::Deserialize;

mod rs {
    pub use crate::core::dice::DExpr;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SaveEffectorRaw {
    #[serde(alias = "A")]
    Advantage,

    #[serde(alias = "D")]
    Disadvantage,

    Bonus {
        bonus: rs::DExpr,

        #[serde(default)]
        uses: Option<NonZeroU32>,
    },
}
