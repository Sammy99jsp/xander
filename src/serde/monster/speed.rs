use std::{num::NonZeroU32, sync::Weak};

use serde::Deserialize;

mod rs {
    pub(crate) use crate::core::stats::{
        monster::speed::{Speed, Speeds},
        stat_block::StatBlock,
    };
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SpeedsRaw {
    WalkingOnly(NonZeroU32),
    Specified {
        walking: NonZeroU32,

        #[serde(default)]
        burrowing: Option<NonZeroU32>,

        #[serde(default)]
        climbing: Option<NonZeroU32>,

        #[serde(default)]
        flying: Option<NonZeroU32>,

        #[serde(default)]
        swimming: Option<NonZeroU32>,
    },
}

impl SpeedsRaw {
    pub fn construct(self, this: Weak<rs::StatBlock>) -> rs::Speeds {
        match self {
            SpeedsRaw::WalkingOnly(walking) => rs::Speeds {
                walking: rs::Speed::new(this.clone(), Some(walking.get())),
                burrowing: rs::Speed::empty(this.clone()),
                climbing: rs::Speed::empty(this.clone()),
                flying: rs::Speed::empty(this.clone()),
                swimming: rs::Speed::empty(this),
            },
            SpeedsRaw::Specified {
                walking,
                burrowing,
                climbing,
                flying,
                swimming,
            } => rs::Speeds {
                walking: rs::Speed::new(this.clone(), Some(walking.get())),
                burrowing: rs::Speed::new(this.clone(), burrowing.map(NonZeroU32::get)),
                climbing: rs::Speed::new(this.clone(), climbing.map(NonZeroU32::get)),
                flying: rs::Speed::new(this.clone(), flying.map(NonZeroU32::get)),
                swimming: rs::Speed::new(this.clone(), swimming.map(NonZeroU32::get)),
            },
        }
    }
}
