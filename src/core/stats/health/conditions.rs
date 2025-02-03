//! Adds all conditions featured in
//! PH-A (pg. 358 SRD 5.1E).

use std::{
    fmt::Debug,
    marker::PhantomData,
    num::NonZeroU8,
    ptr,
    rc::{Rc, Weak},
};

use crate::{
    core::{
        cause::Cause,
        stats::{damage::Immunity, stat_block::StatBlock},
    },
    proxy_wrapper,
    utils::{
        meta::Meta,
        proxy::Dispatch,
        reactive::{Ephemeral, Lifespan, RSlot},
        Proxy, ProxyPart,
    },
};

macro_rules! condition {
    {$id: ident, $($field: ident : $value: expr),* $(,)? } => {
        #[derive(Debug, Clone, Copy)]
        pub struct $id;

        impl Condition for $id {
            const META: &'static ConditionMeta = &ConditionMeta {
                effect: <$id as ConditionEffect>::apply,
                $($field : $value),*
            };
        }

        impl Meta<ConditionMeta> for $id {
            #[inline(always)]
            fn meta(&self) -> &'static ConditionMeta {
                <Self as Condition>::META
            }
        }

        impl std::ops::Deref for $id {
            type Target = ConditionMeta;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.meta()
            }
        }
    }
}

#[derive(Debug)]
pub struct ConditionMeta {
    name: &'static str,
    index: usize,
    effect: fn(&StatBlock, Lifespan),
}

pub trait Condition: ConditionEffect + Meta<ConditionMeta> {
    const META: &'static ConditionMeta;
}

/// The effect a condition has to a creature.
pub trait ConditionEffect {
    #[allow(unused)]
    fn apply(target: &StatBlock, time: Lifespan) {
        unimplemented!()
    }
}

/// An instance of a condition applied
/// to a creature.
#[derive(Debug)]
pub struct ConditionApplication {
    pub lifespan: Lifespan<dyn Cause>,
    pub condition: &'static ConditionMeta,
}

impl Ephemeral for ConditionApplication {
    fn is_alive(&self) -> bool {
        self.lifespan.is_alive()
    }
}

use self::ConditionApplication as Effect;

use super::ConditionApplicationResult;

type EffectSlot = RSlot<Rc<Effect>>;
#[derive(Debug)]
pub struct ConditionStatus {
    weak: Weak<StatBlock>,

    blinded: EffectSlot,
    charmed: EffectSlot,
    deafened: EffectSlot,
    exhaustion: EffectSlot,
    frightened: EffectSlot,
    grappled: EffectSlot,
    incapacitated: EffectSlot,
    invisible: EffectSlot,
    paralyzed: EffectSlot,
    petrified: EffectSlot,
    poisoned: EffectSlot,
    prone: EffectSlot,
    restrained: EffectSlot,
    stunned: EffectSlot,
    unconscious: EffectSlot,
}

impl ConditionStatus {
    pub const fn new(weak: Weak<StatBlock>) -> Self {
        Self {
            weak,
            blinded: RSlot::new(),
            charmed: RSlot::new(),
            deafened: RSlot::new(),
            exhaustion: RSlot::new(),
            frightened: RSlot::new(),
            grappled: RSlot::new(),
            incapacitated: RSlot::new(),
            invisible: RSlot::new(),
            paralyzed: RSlot::new(),
            petrified: RSlot::new(),
            poisoned: RSlot::new(),
            prone: RSlot::new(),
            restrained: RSlot::new(),
            stunned: RSlot::new(),
            unconscious: RSlot::new(),
        }
    }

    /// Apply a [ConditionEffect] to this creature.
    ///
    /// Note: we should have already checked for immunity
    pub fn apply(&mut self, effect: Effect) {
        let current_cond = unsafe {
            // SAFETY: All effects
            let ptr = ptr::addr_of!(self.blinded).add(effect.condition.index) as *mut EffectSlot;

            // This must be done, so that drop() is called on the old value.
            // SAFETY: Not null, or uninitialized at any point.
            ptr.as_mut_unchecked()
        };

        let effect = Rc::new(effect);

        let effect_fn = effect.condition.effect;
        let lifespan = Lifespan::of(&effect);

        current_cond.replace(effect);

        // Call the effect function.
        {
            let stat_block = self.weak.upgrade().expect("Stat block to always exist!");
            effect_fn(&stat_block, lifespan)
        }
    }
}

#[derive(Clone)]
pub struct ConditionImmunity<C> {
    lifespan: Lifespan<dyn Cause>,
    priority: u32,
    immune: bool,
    __: PhantomData<C>,
}

impl<C: Condition> ConditionImmunity<C> {
    pub fn is_immune(&self) -> bool {
        self.immune
    }
}

impl<C: Condition> Default for ConditionImmunity<C> {
    fn default() -> Self {
        Self {
            lifespan: Default::default(),
            priority: Default::default(),
            immune: Default::default(),
            __: Default::default(),
        }
    }
}

impl<C: Condition> std::fmt::Debug for ConditionImmunity<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConditionImmunity")
            .field("lifespan", &self.lifespan)
            .field("priority", &self.priority)
            .field("immune", &self.immune)
            .finish()
    }
}

proxy_wrapper!(ConditionImmunityP<C: Condition>, Proxy<StatBlock, ConditionImmunity<C>>);

#[derive(Debug)]
pub struct ConditionImmunities {
    pub(crate) weak: Weak<StatBlock>,

    pub(crate) blinded: ConditionImmunityP<Blinded>,
    pub(crate) charmed: ConditionImmunityP<Charmed>,
    pub(crate) deafened: ConditionImmunityP<Deafened>,
    pub(crate) exhaustion: ConditionImmunityP<Exhaustion>,
    pub(crate) frightened: ConditionImmunityP<Frightened>,
    pub(crate) grappled: ConditionImmunityP<Grappled>,
    pub(crate) incapacitated: ConditionImmunityP<Incapacitated>,
    pub(crate) invisible: ConditionImmunityP<Invisible>,
    pub(crate) paralyzed: ConditionImmunityP<Paralyzed>,
    pub(crate) petrified: ConditionImmunityP<Petrified>,
    pub(crate) poisoned: ConditionImmunityP<Poisoned>,
    pub(crate) prone: ConditionImmunityP<Prone>,
    pub(crate) restrained: ConditionImmunityP<Restrained>,
    pub(crate) stunned: ConditionImmunityP<Stunned>,
    pub(crate) unconscious: ConditionImmunityP<Unconscious>,
}

impl<C: Condition> ConditionImmunityP<C> {
    pub fn new(weak: Weak<StatBlock>) -> Self {
        Self(Proxy::new(ConditionImmunity::default(), weak), PhantomData)
    }
}

// TODO: Move to a more in-depth struct, with the [Lifespan]
impl<C: Condition> ProxyPart<StatBlock, ConditionImmunity<C>> for Immunity {
    fn compute(&mut self, _: &StatBlock, prev: &mut ConditionImmunity<C>, _: Dispatch<'_>) {
        prev.immune = true;
    }
}

impl ConditionImmunities {
    pub fn new(weak: Weak<StatBlock>) -> Self {
        Self {
            weak: weak.clone(),
            blinded: ConditionImmunityP::new(weak.clone()),
            charmed: ConditionImmunityP::new(weak.clone()),
            deafened: ConditionImmunityP::new(weak.clone()),
            exhaustion: ConditionImmunityP::new(weak.clone()),
            frightened: ConditionImmunityP::new(weak.clone()),
            grappled: ConditionImmunityP::new(weak.clone()),
            incapacitated: ConditionImmunityP::new(weak.clone()),
            invisible: ConditionImmunityP::new(weak.clone()),
            paralyzed: ConditionImmunityP::new(weak.clone()),
            petrified: ConditionImmunityP::new(weak.clone()),
            poisoned: ConditionImmunityP::new(weak.clone()),
            prone: ConditionImmunityP::new(weak.clone()),
            restrained: ConditionImmunityP::new(weak.clone()),
            stunned: ConditionImmunityP::new(weak.clone()),
            unconscious: ConditionImmunityP::new(weak),
        }
    }

    pub(super) fn is_immune(&self, meta: &'static ConditionMeta) -> ConditionApplicationResult {
        // SAFETY: All indices are inside this struct!
        let proxy = unsafe {
            let start = ptr::addr_of!(self.blinded) as *const ConditionImmunityP<()>;
            start.add(meta.index).as_ref_unchecked()
        };

        match proxy.get() {
            ConditionImmunity {
                lifespan,
                immune: true,
                ..
            } => ConditionApplicationResult::Unsuccessful(lifespan),
            _ => ConditionApplicationResult::Successful,
        }
    }
}
/* CONDITIONS */

condition! {
    Blinded,
    name: "Blinded",
    index: 0,
}

impl ConditionEffect for Blinded {}

condition! {
    Charmed,
    name: "Charmed",
    index: 1,
}

impl ConditionEffect for Charmed {}

condition! {
    Deafened,
    name: "Deafened",
    index: 2,
}

impl ConditionEffect for Deafened {}

/* EXHAUSTION */

// We use a NonZeroU8 for niche optimization,
// and exploit the fact that level 0 exhaustion == no exhaustion
// to save on memory.
#[derive(Debug, Clone, Copy)]
pub struct Exhaustion(NonZeroU8);

impl Condition for Exhaustion {
    const META: &'static ConditionMeta = &ConditionMeta {
        name: "Exhaustion",
        index: 3,
        effect: <Self as ConditionEffect>::apply,
    };
}

impl ConditionEffect for Exhaustion {}

impl std::ops::Deref for Exhaustion {
    type Target = ConditionMeta;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.meta()
    }
}

impl Meta<ConditionMeta> for Exhaustion {
    #[inline(always)]
    fn meta(&self) -> &'static ConditionMeta {
        <Self as Condition>::META
    }
}

/* /EXHAUSTION  */

condition! {
    Frightened,
    name: "Frightened",
    index: 4,
}

impl ConditionEffect for Frightened {}

condition! {
    Grappled,
    name: "Grappled",
    index: 5,
}

impl ConditionEffect for Grappled {}

condition! {
    Incapacitated,
    name: "Incapacitated",
    index: 6,
}

impl ConditionEffect for Incapacitated {}

condition! {
    Invisible,
    name: "Invisible",
    index: 7,
}

impl ConditionEffect for Invisible {}

condition! {
    Paralyzed,
    name: "Paralyzed",
    index: 8,
}

impl ConditionEffect for Paralyzed {}

condition! {
    Petrified,
    name: "Petrified",
    index: 9,
}

impl ConditionEffect for Petrified {}

condition! {
    Poisoned,
    name: "Poisoned",
    index: 10,
}

impl ConditionEffect for Poisoned {}

condition! {
    Prone,
    name: "Prone",
    index: 11,
}

impl ConditionEffect for Prone {}

condition! {
    Restrained,
    name: "Restrained",
    index: 12,
}

impl ConditionEffect for Restrained {}

condition! {
    Stunned,
    name: "Stunned",
    index: 13,
}

impl ConditionEffect for Stunned {}

condition! {
    Unconscious,
    name: "Unconscious",
    index: 14,
}

impl ConditionEffect for Unconscious {}

/* /CONDITIONS */

impl std::fmt::Display for ConditionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(blinded) = self.blinded.get() {
            write!(f, "{blinded:?}");
        }
        if let Some(charmed) = self.charmed.get() {
            write!(f, "{charmed:?}");
        }
        if let Some(deafened) = self.deafened.get() {
            write!(f, "{deafened:?}");
        }
        if let Some(exhaustion) = self.exhaustion.get() {
            write!(f, "{exhaustion:?}");
        }
        if let Some(frightened) = self.frightened.get() {
            write!(f, "{frightened:?}");
        }
        if let Some(grappled) = self.grappled.get() {
            write!(f, "{grappled:?}");
        }
        if let Some(incapacitated) = self.incapacitated.get() {
            write!(f, "{incapacitated:?}");
        }
        if let Some(invisible) = self.invisible.get() {
            write!(f, "{invisible:?}");
        }
        if let Some(paralyzed) = self.paralyzed.get() {
            write!(f, "{paralyzed:?}");
        }
        if let Some(petrified) = self.petrified.get() {
            write!(f, "{petrified:?}");
        }
        if let Some(poisoned) = self.poisoned.get() {
            write!(f, "{poisoned:?}");
        }
        if let Some(prone) = self.prone.get() {
            write!(f, "{prone:?}");
        }
        if let Some(restrained) = self.restrained.get() {
            write!(f, "{restrained:?}");
        }
        if let Some(stunned) = self.stunned.get() {
            write!(f, "{stunned:?}");
        }
        if let Some(unconscious) = self.unconscious.get() {
            write!(f, "{unconscious:?}");
        }

        Ok(())
    }
}
