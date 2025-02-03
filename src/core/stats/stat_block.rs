use std::{cell::Cell, marker::PhantomData, ops::Index, rc::Weak};

use serde::Deserialize;

use crate::{
    core::{
        dice::{DEvalTree, DExpr},
        stats::{
            abilities::{
                Ability, Charisma as CHA, Constitution as CON, Dexterity as DEX,
                Intelligence as INT, Strength as STR, Wisdom as WIS,
            },
            skills::*,
            AbilityScore,
        },
    },
    utils::{proxy::Dispatch, Proxy, ProxyPart},
};

use super::{
    ac::AC,
    damage::{self, Damage, DamageHandling, DamagePart, DamageType},
    health::{
        conditions::{ConditionApplication, ConditionImmunities},
        ConditionApplicationResult, DamageTaken, Health, TempHP,
    },
    monsters::Monster,
};

#[macro_export]
macro_rules! proxy_wrapper {
    ($id: ident $(<$gen: ident : $tr: path>)?, $inner: ty) => {
        #[repr(transparent)]
        pub struct $id$(<$gen>)?($inner $(,PhantomData<$gen>)?);

        impl $(<$gen: $tr>)? std::fmt::Debug for$id $(<$gen>)?  {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl $(<$gen>)? std::ops::Deref for $id $(<$gen>)? {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $(<$gen>)? std::ops::DerefMut for $id $(<$gen>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[derive(Debug)]
pub struct StatBlock {
    pub name: String,
    pub ty: CreatureType,
    pub size: Size,

    pub scores: AbilityScores,
    pub modifiers: AbilityModifiers,
    pub skills: Skills,
    pub damage_effectors: DamageEffectors,
    pub condition_immunities: ConditionImmunities,

    pub health: Health,
    pub ac: AC,

    pub proficiency_bonus: ProficiencyBonus,

    /// Is this creature dead?
    pub dead: Cell<Option<Dead>>,
}
impl StatBlock {
    /// Heal this creature (up to its max HP).
    pub fn heal(&self, health: u32) {
        self.health.hp.heal(health)
    }

    /// Make this creature take some damage,
    /// returning the damage that was taken.
    pub fn damage(&self, damage: Damage) -> DamageTaken {
        self.health.take_damage(damage)
    }

    /// Attempt to apply a condition (via [ConditionApplication])
    /// to this creature, returning if it 
    pub fn apply(&self, cond: ConditionApplication) -> ConditionApplicationResult {
        self.health.apply_condition(cond)
    }

    pub fn ac(&self) -> &AC {
        &self.ac
    }

    /// Get the current HP for this creature.
    pub fn hp(&self) -> u32 {
        self.health.current_hp()
    }

    /// Get the creature's max HP.
    pub fn max_hp(&self) -> u32 {
        self.health.max_hp()
    }

    /// Get the temporary HP for this creature, if any.
    pub fn temp_hp(&self) -> Option<u32> {
        self.health
            .hp
            .temp
            .borrow()
            .get()
            .map(|TempHP { amount, .. }| *amount)
    }

 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

/// A marker that signifies this creature has died.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dead;

proxy_wrapper!(ProficiencyBonus, Proxy<StatBlock, DExpr>);

impl ProficiencyBonus {
    pub fn derived(this: Weak<StatBlock>) -> Self {
        Self(Proxy::derived(
            |this| match &this.ty {
                CreatureType::Player => todo!(),
                CreatureType::Monster(Monster { cr, .. }) => cr.proficiency_bonus(),
            },
            this,
        ))
    }

    pub fn fixed(this: Weak<StatBlock>, val: DExpr) -> Self {
        Self(Proxy::new(val, this))
    }
}

#[derive(Debug)]
pub struct Proficiency;

impl ProxyPart<StatBlock, DExpr> for Proficiency {
    fn compute(&mut self, ctx: &StatBlock, prev: &mut DExpr, _: Dispatch<'_>) {
        *prev += ctx.proficiency_bonus.get();
    }
}

#[derive(Debug)]
pub struct Override(pub i32);

impl ProxyPart<StatBlock, DExpr> for Override {
    fn compute(&mut self, _: &StatBlock, prev: &mut DExpr, _: Dispatch<'_>) {
        *prev = DExpr::Constant(self.0);
    }
}

#[derive(Debug)]
pub enum CreatureType {
    Player,
    Monster(Monster),
}

proxy_wrapper!(AbilityScoreP<A: Ability>, Proxy<StatBlock, DExpr>);

#[allow(unused)]
#[repr(C)]
#[derive(Debug)]
pub struct AbilityScores {
    pub strength: AbilityScoreP<STR>,
    pub dexterity: AbilityScoreP<DEX>,
    pub constitution: AbilityScoreP<CON>,
    pub intelligence: AbilityScoreP<INT>,
    pub wisdom: AbilityScoreP<WIS>,
    pub charisma: AbilityScoreP<CHA>,
}

impl<A: Ability> AbilityScoreP<A> {
    pub fn new(ctx: Weak<StatBlock>, score: AbilityScore) -> Self {
        Self(Proxy::new(score.into(), ctx), PhantomData)
    }
}

impl AbilityScores {
    pub fn new(
        ctx: &Weak<StatBlock>,
        str: AbilityScore,
        dex: AbilityScore,
        con: AbilityScore,
        int: AbilityScore,
        wis: AbilityScore,
        cha: AbilityScore,
    ) -> Self {
        Self {
            strength: AbilityScoreP::new(ctx.clone(), str),
            dexterity: AbilityScoreP::new(ctx.clone(), dex),
            constitution: AbilityScoreP::new(ctx.clone(), con),
            intelligence: AbilityScoreP::new(ctx.clone(), int),
            wisdom: AbilityScoreP::new(ctx.clone(), wis),
            charisma: AbilityScoreP::new(ctx.clone(), cha),
        }
    }

    ///
    /// ### Safety
    /// The index must be a valid valid [Ability::META] index.
    ///
    #[inline(always)]
    pub(super) const unsafe fn get_index(&self, index: usize) -> &AbilityScoreP<()> {
        let arr = self as *const Self as *const AbilityScoreP<()>;
        // SAFETY: Fields are aligned to 64 bytes, so our pointer arithmetic should be fine.
        unsafe {
            arr.add(index)
                .cast::<AbilityScoreP<()>>()
                .as_ref_unchecked()
        }
    }

    #[inline(always)]
    pub const fn get<A: Ability>(&self) -> &AbilityScoreP<A> {
        let idx = const { A::META.index() };
        // SAFETY:  (a) idx is a valid Ability::META index.
        //          (b) size_of::<AbilityScoreP<()>>() == size_of::<AbilityScoreP<A>>()
        unsafe { (self.get_index(idx) as *const _ as *const AbilityScoreP<A>).as_ref_unchecked() }
    }
}

impl<A: Ability> Index<A> for AbilityScores {
    type Output = AbilityScoreP<A>;

    #[inline]
    fn index(&self, _: A) -> &Self::Output {
        self.get::<A>()
    }
}

proxy_wrapper!(AbilityModifierP<A: Ability>, Proxy<StatBlock, DExpr>);

#[allow(unused)]
#[repr(C)]
#[derive(Debug)]
pub struct AbilityModifiers {
    strength: AbilityModifierP<STR>,
    dexterity: AbilityModifierP<DEX>,
    constitution: AbilityModifierP<CON>,
    intelligence: AbilityModifierP<INT>,
    wisdom: AbilityModifierP<WIS>,
    charisma: AbilityModifierP<CHA>,
}

impl<A: Ability> AbilityModifierP<A> {
    pub fn new(ctx: Weak<StatBlock>) -> Self {
        Self(
            Proxy::derived(
                |stats| {
                    let score =
                        AbilityScore::new(stats.scores.get::<A>().get().result() as u8).unwrap();
                    score.modifier().into()
                },
                ctx,
            ),
            PhantomData,
        )
    }
}

impl AbilityModifiers {
    // TODO: Replace maybe with a proc macro.
    pub fn new(ctx: &Weak<StatBlock>) -> Self {
        Self {
            strength: AbilityModifierP::new(ctx.clone()),
            dexterity: AbilityModifierP::new(ctx.clone()),
            constitution: AbilityModifierP::new(ctx.clone()),
            intelligence: AbilityModifierP::new(ctx.clone()),
            wisdom: AbilityModifierP::new(ctx.clone()),
            charisma: AbilityModifierP::new(ctx.clone()),
        }
    }

    ///
    /// ### Safety
    /// The index must be a valid valid [Ability::META] index.
    ///
    #[inline(always)]
    pub(super) const unsafe fn get_index(&self, index: usize) -> &AbilityModifierP<()> {
        let arr = self as *const Self as *const AbilityModifierP<()>;
        // SAFETY: Fields are aligned to 64 bytes, so our pointer arithmetic should be fine.
        unsafe {
            arr.add(index)
                .cast::<AbilityModifierP<()>>()
                .as_ref_unchecked()
        }
    }

    #[inline(always)]
    pub const fn get<A: Ability>(&self) -> &AbilityModifierP<A> {
        let idx = A::META.index();
        // SAFETY:  (a) idx is a valid Ability::META index.
        //          (b) size_of::<AbilityModifierP<()>>() == size_of::<AbilityModifierP<A>>()
        unsafe {
            (self.get_index(idx) as *const _ as *const AbilityModifierP<A>).as_ref_unchecked()
        }
    }
}

impl<A: Ability> Index<A> for AbilityModifiers {
    type Output = AbilityModifierP<A>;

    #[inline]
    fn index(&self, _: A) -> &Self::Output {
        self.get::<A>()
    }
}

proxy_wrapper!(SkillP<S: Skill>, Proxy<StatBlock, DExpr>);

#[allow(unused)]
#[derive(Debug)]
#[repr(C)]
pub struct Skills {
    pub athletics: SkillP<Athletics>,
    pub acrobatics: SkillP<Acrobatics>,
    pub sleight_of_hand: SkillP<SleightOfHand>,
    pub stealth: SkillP<Stealth>,
    pub arcana: SkillP<Arcana>,
    pub history: SkillP<History>,
    pub investigation: SkillP<Investigation>,
    pub nature: SkillP<Nature>,
    pub religion: SkillP<Religion>,
    pub animal_handling: SkillP<AnimalHandling>,
    pub insight: SkillP<Insight>,
    pub medicine: SkillP<Medicine>,
    pub perception: SkillP<Perception>,
    pub survival: SkillP<Survival>,
    pub deception: SkillP<Deception>,
    pub intimidation: SkillP<Intimidation>,
    pub performance: SkillP<Performance>,
    pub persuasion: SkillP<Persuasion>,
}

impl Skills {
    pub fn new(ctx: &Weak<StatBlock>) -> Self {
        Self {
            athletics: SkillP::new(ctx.clone()),
            acrobatics: SkillP::new(ctx.clone()),
            sleight_of_hand: SkillP::new(ctx.clone()),
            stealth: SkillP::new(ctx.clone()),
            arcana: SkillP::new(ctx.clone()),
            history: SkillP::new(ctx.clone()),
            investigation: SkillP::new(ctx.clone()),
            nature: SkillP::new(ctx.clone()),
            religion: SkillP::new(ctx.clone()),
            animal_handling: SkillP::new(ctx.clone()),
            insight: SkillP::new(ctx.clone()),
            medicine: SkillP::new(ctx.clone()),
            perception: SkillP::new(ctx.clone()),
            survival: SkillP::new(ctx.clone()),
            deception: SkillP::new(ctx.clone()),
            intimidation: SkillP::new(ctx.clone()),
            performance: SkillP::new(ctx.clone()),
            persuasion: SkillP::new(ctx.clone()),
        }
    }

    ///
    /// ### Safety
    /// The index must be a valid valid [Skill::META] index.
    ///
    #[inline(always)]
    pub(super) const unsafe fn get_index(&self, index: usize) -> &SkillP<()> {
        let arr = self as *const Self as *const SkillP<()>;
        // SAFETY: Fields are aligned to 64 bytes, so our pointer arithmetic should be fine.
        unsafe { arr.add(index).cast::<SkillP<()>>().as_ref_unchecked() }
    }

    #[inline(always)]
    pub const fn get<S: Skill>(&self) -> &SkillP<S> {
        let idx = S::META.index();
        // SAFETY:  (a) idx is a valid Skill::META index.
        //          (b) size_of::<SkillP<()>>() == size_of::<SkillP<S>>()
        unsafe { (self.get_index(idx) as *const _ as *const SkillP<S>).as_ref_unchecked() }
    }
}

impl<S: Skill> Index<S> for Skills {
    type Output = SkillP<S>;

    #[inline]
    fn index(&self, _: S) -> &Self::Output {
        self.get::<S>()
    }
}

impl<S: Skill> SkillP<S> {
    pub fn new(ctx: Weak<StatBlock>) -> Self {
        Self(
            Proxy::derived(|ctx| ctx.modifiers.get::<S::Base>().get(), ctx),
            PhantomData,
        )
    }
}

proxy_wrapper!(DamageP<D: DamageType>, Proxy<StatBlock, DamageHandling>);

/// Handles Resistance, Vulnerability, and Immunity.
///
/// The main function for this struct is [DamageEffectors::calculate].
#[allow(unused)]
#[derive(Debug)]
#[repr(C)]
pub struct DamageEffectors {
    pub acid: DamageP<damage::Acid>,
    pub bludgeoning: DamageP<damage::Bludgeoning>,
    pub cold: DamageP<damage::Cold>,
    pub fire: DamageP<damage::Fire>,
    pub force: DamageP<damage::Force>,
    pub lightning: DamageP<damage::Lightning>,
    pub necrotic: DamageP<damage::Necrotic>,
    pub piercing: DamageP<damage::Piercing>,
    pub poison: DamageP<damage::Poison>,
    pub psychic: DamageP<damage::Psychic>,
    pub radiant: DamageP<damage::Radiant>,
    pub slashing: DamageP<damage::Slashing>,
    pub thunder: DamageP<damage::Thunder>,

    /// For custom predicates not necessarily dependent
    /// on the damage type itself, like 'non-magical' damage.
    pub __all__: Vec<(fn(&DamagePart) -> bool, fn(&mut DamageHandling))>,
}

impl DamageEffectors {
    pub fn empty(ctx: &Weak<StatBlock>) -> Self {
        Self {
            acid: DamageP::new(ctx.clone()),
            bludgeoning: DamageP::new(ctx.clone()),
            cold: DamageP::new(ctx.clone()),
            fire: DamageP::new(ctx.clone()),
            force: DamageP::new(ctx.clone()),
            lightning: DamageP::new(ctx.clone()),
            necrotic: DamageP::new(ctx.clone()),
            piercing: DamageP::new(ctx.clone()),
            poison: DamageP::new(ctx.clone()),
            psychic: DamageP::new(ctx.clone()),
            radiant: DamageP::new(ctx.clone()),
            slashing: DamageP::new(ctx.clone()),
            thunder: DamageP::new(ctx.clone()),
            __all__: Vec::with_capacity(4),
        }
    }

    ///
    /// ### Safety
    /// The index must be a valid valid [DamageType::META] index.
    ///
    #[inline(always)]
    pub(super) const unsafe fn get_index(&self, index: usize) -> &DamageP<()> {
        let arr = self as *const Self as *const DamageP<()>;
        // SAFETY: Fields are aligned to 64 bytes, so our pointer arithmetic should be fine.
        arr.add(index).cast::<DamageP<()>>().as_ref_unchecked()
    }

    ///
    /// ### Safety
    /// The index must be a valid valid [DamageType::META] index.
    ///
    #[inline(always)]
    const fn get_mut<D: DamageType>(&mut self) -> &mut DamageP<D> {
        let arr = self as *mut Self as *mut DamageP<()>;
        // SAFETY: Fields are aligned to 64 bytes, so our pointer arithmetic should be fine.
        unsafe {
            arr.add(D::META.index())
                .cast::<DamageP<D>>()
                .as_mut_unchecked()
        }
    }

    /// Calculate the damage that this entity needs to take,
    /// after accounting for all resistances, vulnerabilities, and immunities.
    pub(super) fn calculate(&self, damage: Damage) -> Damage {
        let parts = damage
            .0
            .into_iter()
            .map(|mut dmg| {
                // Put through "all damage" handlers first (i.e. "nom-magical" damage, etc.)
                let handling = self.__all__.iter().fold(
                    DamageHandling::default(),
                    |mut handling, (pred, next)| {
                        if pred(&mut dmg) {
                            next(&mut handling);
                        }

                        handling
                    },
                );

                // SAFETY: Damage will always contain a valid reference to a `DamageTypeMeta`,
                //         which have indexes 0..=13, which are valid for this struct.
                let handling = handling | unsafe { self.get_index(dmg.damage_type.index()) }.get();

                // Actually apply the effects now.
                if handling.immunity {
                    dmg.amount =
                        Box::new(DEvalTree::Mul(dmg.amount, Box::new(DEvalTree::Modifier(0))));
                }

                // Apply resistance before vulnerability, according to the SRD 5.1E pg. 97
                if handling.resistance {
                    dmg.amount =
                        Box::new(DEvalTree::Div(dmg.amount, Box::new(DEvalTree::Modifier(2))));
                }

                if handling.vulnerability {
                    dmg.amount =
                        Box::new(DEvalTree::Mul(dmg.amount, Box::new(DEvalTree::Modifier(2))));
                }

                dmg
            })
            .collect();

        Damage(parts)
    }

    pub fn add_effect<D: DamageType, P: ProxyPart<StatBlock, DamageHandling> + 'static>(
        &mut self,
        _type: D,
        effect: P,
    ) {
        let prox = self.get_mut::<D>();
        prox.insert(effect);
    }
}

impl<D: DamageType> DamageP<D> {
    pub fn new(ctx: Weak<StatBlock>) -> Self {
        Self(Proxy::new(DamageHandling::default(), ctx), PhantomData)
    }
}
