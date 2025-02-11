use std::{ptr, sync::Weak};

use crate::{
    core::stats::stat_block::StatBlock,
    proxy_wrapper,
    utils::{
        meta::{meta_trait, Meta},
        Proxy,
    },
};

/// 
/// ### Safety
/// Check to see if the index is not usize::META.
/// 
#[derive(Debug)]
pub struct SpeedTypeMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub(crate) index: usize,
}

pub trait SpeedType: Meta<SpeedTypeMeta> {
    const META: &'static SpeedTypeMeta;
}

proxy_wrapper!(Speed, Proxy<StatBlock, Option<u32>>);

#[derive(Debug)]
pub struct Speeds {
    pub walking: Speed,
    pub burrowing: Speed,
    pub climbing: Speed,
    pub flying: Speed,
    pub swimming: Speed,
}

impl Speeds {
    /// Get the speed of a particular movement type.
    pub fn of_type(&self, mode: &'static SpeedTypeMeta) -> Option<u32> {
        // Prevent out of bounds access.
        if ptr::addr_eq(mode, Crawling::META) {
            return None;
        }

        // SAFETY: The index is always within bounds (0..5).
        let mode = unsafe {
            let ptr: *const Speed = ptr::addr_of!(self.walking);
            ptr.add(mode.index).as_ref_unchecked()
        };

        mode.get()
    }

    pub fn has(&self, meta: &'static SpeedTypeMeta) -> bool {
        self.of_type(meta).is_some()
    }
}

meta_trait!(speed, SpeedType, SpeedTypeMeta);

impl Speed {
    pub fn new(ctx: Weak<StatBlock>, init: Option<u32>) -> Self {
        Self(Proxy::new(init, ctx))
    }

    pub fn empty(ctx: Weak<StatBlock>) -> Self {
        Self::new(ctx, None)
    }
}

speed!(
    Walking,
    SpeedTypeMeta {
        name: "Walking",
        description: "A monster's speed tells you how far it can move on its turn. All creatures have a walking speed, simply called the monster's speed. Creatures that have no form of ground-based locomotion have a walking speed of 0 feet.",
        index: 0,
    }
);

speed!(
    Burrowing,
    SpeedTypeMeta {
        name: "Burrowing",
        description: "A monster that has a burrowing speed can use that speed to move through sand, earth, mud, or ice. A monster can't burrow through solid rock unless it has a special trait that allows it to do so.",
        index: 1,
    }
);

speed!(
    Climbing,
    SpeedTypeMeta {
        name: "Climbing",
        description: "A monster that has a climbing speed can use all or part of its movement to move on vertical surfaces. The monster doesn't need to spend extra movement to climb.",
        index: 2,
    }
);

speed!(
    Flying,
    SpeedTypeMeta {
        name: "Flying",
        description: "A monster that has a flying speed can use all or part of its movement to fly. Some monsters have the ability to hover, which makes them hard to knock out of the air. Such a monster stops hovering when it dies.",
        index: 3,
    }
);

speed!(
    Swimming,
    SpeedTypeMeta {
        name: "Swimming",
        description:
            "A monster that has a swimming speed doesn't need to spend extra movement to swim.",
        index: 4,
    }
);

// Crawling is special. Since a creature doesn't have a
// dedicated 'crawling' speed, we'll use usize::MAX for the index,
// which will hopefully cause an out-of-bounds memory access and crash,
// rather than a nasty bug.
speed!(
    Crawling,
    SpeedTypeMeta {
        name: "Crawling",
        description:
            "A monster that has a swimming speed doesn't need to spend extra movement to swim.",
        index: usize::MAX,
    }
);

impl std::fmt::Display for Speeds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ft., ",
            self.walking.get().expect("Always has a walking speed.")
        )?;

        [
            ("burrow", &self.burrowing),
            ("climb", &self.climbing),
            ("fly", &self.flying),
            ("swim", &self.swimming),
        ]
        .into_iter()
        .try_for_each(|(ty, speed)| -> std::fmt::Result {
            if let Some(speed) = speed.get() {
                write!(f, "{} {} ft., ", ty, speed)?;
            }

            Ok(())
        })
    }
}
