use std::{collections::HashMap, sync::LazyLock};

mod rs {
    pub(crate) use crate::core::stats::damage::DamageTypeMeta;
}

pub static DAMAGE_TYPES: LazyLock<HashMap<&'static str, &'static rs::DamageTypeMeta>> =
    LazyLock::new(|| {
        use crate::core::stats::damage::*;

        HashMap::from_iter([
            ("acid", Acid::META),
            ("bludgeoning", Bludgeoning::META),
            ("cold", Cold::META),
            ("fire", Fire::META),
            ("force", Force::META),
            ("lightning", Lightning::META),
            ("necrotic", Necrotic::META),
            ("piercing", Piercing::META),
            ("poison", Poison::META),
            ("psychic", Psychic::META),
            ("radiant", Radiant::META),
            ("slashing", Slashing::META),
            ("thunder", Thunder::META),
        ])
    });
