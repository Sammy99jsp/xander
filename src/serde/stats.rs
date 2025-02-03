use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::Display,
    sync::{Arc, LazyLock, RwLock, Weak},
};

use serde::{Deserialize, Deserializer};

use crate::{
    core::{
        dice::DExpr,
        stats::{
            ac::{ACPart, AC},
            damage::{DamageType, Immunity, Resistance, Vulnerability},
            health::{
                conditions::{Condition, ConditionImmunities, ConditionImmunityP, ConditionStatus},
                Health, HitDice, HP,
            },
            skills::Skill,
            stat_block::{
                AbilityModifiers, AbilityScoreP, AbilityScores, CreatureType, DamageEffectors,
                DamageP, Override, Proficiency, ProficiencyBonus, Size, SkillP, Skills, StatBlock,
            },
            AbilityScore,
        },
    },
    utils::{reactive::Lifespan, Proxy},
};

use super::monster::MonsterRaw;

macro_rules! ability_score_from {
    ($method: ident, $ty: ty) => {
        #[allow(clippy::useless_conversion)]
        fn $method<E>(self, v: $ty) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            u8::try_from(v)
                .ok()
                .and_then(AbilityScore::new)
                .ok_or_else(|| E::custom("an ability scores between 1 and 30 inclusive"))
        }
    };
}

impl<'de> Deserialize<'de> for AbilityScore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = AbilityScore;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "An ability score.")
            }

            // We have try from all integer number types. Yikes!
            ability_score_from!(visit_u8, u8);
            ability_score_from!(visit_u16, u16);
            ability_score_from!(visit_u32, u32);
            ability_score_from!(visit_u64, u64);
            ability_score_from!(visit_i8, i8);
            ability_score_from!(visit_i16, i16);
            ability_score_from!(visit_i32, i32);
            ability_score_from!(visit_i64, i64);
        }

        deserializer.deserialize_u8(Visitor)
    }
}

#[derive(Debug, Deserialize)]
pub struct AbilityScoresRaw {
    #[serde(alias = "str")]
    strength: AbilityScore,

    #[serde(alias = "dex")]
    dexterity: AbilityScore,

    #[serde(alias = "con")]
    constitution: AbilityScore,

    #[serde(alias = "int")]
    intelligence: AbilityScore,

    #[serde(alias = "wis")]
    wisdom: AbilityScore,

    #[serde(alias = "cha")]
    charisma: AbilityScore,
}

impl AbilityScoresRaw {
    pub fn construct(self, this: Weak<StatBlock>) -> AbilityScores {
        let Self {
            strength,
            dexterity,
            constitution,
            intelligence,
            wisdom,
            charisma,
        } = self;

        AbilityScores {
            strength: AbilityScoreP::new(this.clone(), strength),
            dexterity: AbilityScoreP::new(this.clone(), dexterity),
            constitution: AbilityScoreP::new(this.clone(), constitution),
            intelligence: AbilityScoreP::new(this.clone(), intelligence),
            wisdom: AbilityScoreP::new(this.clone(), wisdom),
            charisma: AbilityScoreP::new(this.clone(), charisma),
        }
    }
}

// In theory, an extension/plugin could add to this before we parse...
type DerivedModType = fn(&mut SkillP<()>);
static DERIVED_MODS: LazyLock<HashMap<&'static str, DerivedModType>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("P", (|s| s.insert(Proficiency)) as DerivedModType);
    map.insert("proficiency", |s| s.insert(Proficiency));
    map
});

#[derive(Debug, Deserialize)]
#[serde(try_from = "ModOrOverrideRaw")]
pub enum ModOrOverride {
    Derived(DerivedModType),
    Override(i32),
}

pub struct UnrecognizedModifierType;

impl Display for UnrecognizedModifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized modifier: expected either a derived modifier, such as 'proficiency' | 'P', etc.; \
        or a fixed modifier as a integer or string (ex. '+4' | +4 | 4)")
    }
}

impl TryFrom<ModOrOverrideRaw> for ModOrOverride {
    type Error = UnrecognizedModifierType;

    fn try_from(value: ModOrOverrideRaw) -> Result<Self, Self::Error> {
        match value {
            ModOrOverrideRaw::Str(s) => {
                // Integer with a plus sign inside string.
                let num: Option<Result<i32, _>> = s.strip_prefix("+").map(|s| s.parse());
                if let Some(Ok(num)) = num {
                    return Ok(Self::Override(num));
                }

                // Parse as integer inside string.
                let num: Result<i32, _> = s[1..].parse();
                if let Ok(num) = num {
                    return Ok(Self::Override(num));
                }

                // Derived Mod?
                DERIVED_MODS
                    .get(s.as_str())
                    .copied()
                    .map(Self::Derived)
                    .ok_or(UnrecognizedModifierType)
            }
            ModOrOverrideRaw::Modifier(modifier) => Ok(Self::Override(modifier)),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ModOrOverrideRaw {
    Str(String),
    Modifier(i32),
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct SkillsRaw {
    athletics: Option<ModOrOverride>,
    acrobatics: Option<ModOrOverride>,
    sleight_of_hand: Option<ModOrOverride>,
    stealth: Option<ModOrOverride>,
    arcana: Option<ModOrOverride>,
    history: Option<ModOrOverride>,
    investigation: Option<ModOrOverride>,
    nature: Option<ModOrOverride>,
    religion: Option<ModOrOverride>,
    animal_handling: Option<ModOrOverride>,
    insight: Option<ModOrOverride>,
    medicine: Option<ModOrOverride>,
    perception: Option<ModOrOverride>,
    survival: Option<ModOrOverride>,
    deception: Option<ModOrOverride>,
    intimidation: Option<ModOrOverride>,
    performance: Option<ModOrOverride>,
    persuasion: Option<ModOrOverride>,
}

impl SkillsRaw {
    pub fn construct(self, this: Weak<StatBlock>) -> Skills {
        let Self {
            athletics,
            acrobatics,
            sleight_of_hand,
            stealth,
            arcana,
            history,
            investigation,
            nature,
            religion,
            animal_handling,
            insight,
            medicine,
            perception,
            survival,
            deception,
            intimidation,
            performance,
            persuasion,
        } = self;

        fn do_thing<S: Skill>(this: &Weak<StatBlock>, opt: Option<ModOrOverride>) -> SkillP<S> {
            let mut proxy = SkillP::new(this.clone());

            match opt {
                None => (),
                Some(ModOrOverride::Derived(func)) => unsafe {
                    // SAFETY: SkillP<T> does not differ in layout, since
                    //         T is only in PhantomData<T>.
                    let tmp = (&mut proxy as *mut _ as *mut SkillP<()>).as_mut_unchecked();
                    func(tmp);
                },
                Some(ModOrOverride::Override(ov)) => {
                    proxy.insert(Override(ov));
                }
            }

            proxy
        }

        let this = &this;
        Skills {
            athletics: do_thing(this, athletics),
            acrobatics: do_thing(this, acrobatics),
            sleight_of_hand: do_thing(this, sleight_of_hand),
            stealth: do_thing(this, stealth),
            arcana: do_thing(this, arcana),
            history: do_thing(this, history),
            investigation: do_thing(this, investigation),
            nature: do_thing(this, nature),
            religion: do_thing(this, religion),
            animal_handling: do_thing(this, animal_handling),
            insight: do_thing(this, insight),
            medicine: do_thing(this, medicine),
            perception: do_thing(this, perception),
            survival: do_thing(this, survival),
            deception: do_thing(this, deception),
            intimidation: do_thing(this, intimidation),
            performance: do_thing(this, performance),
            persuasion: do_thing(this, persuasion),
        }
    }
}

type DamageResponseType = fn(&mut DamageP<()>);
static DAMAGE_RESPONSES: LazyLock<HashMap<&'static str, DamageResponseType>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        map.insert("R", (|s| s.insert(Resistance)) as DamageResponseType);
        map.insert("resistance", |s| s.insert(Resistance));
        map.insert("V", (|s| s.insert(Vulnerability)) as DamageResponseType);
        map.insert("vulnerability", |s| s.insert(Vulnerability));
        map.insert("I", (|s| s.insert(Immunity)) as DamageResponseType);
        map.insert("immunity", |s| s.insert(Immunity));
        map
    });

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct DamageEffectorsRaw {
    acid: Option<DamageResponse>,
    bludgeoning: Option<DamageResponse>,
    cold: Option<DamageResponse>,
    fire: Option<DamageResponse>,
    force: Option<DamageResponse>,
    lightning: Option<DamageResponse>,
    necrotic: Option<DamageResponse>,
    piercing: Option<DamageResponse>,
    poison: Option<DamageResponse>,
    psychic: Option<DamageResponse>,
    radiant: Option<DamageResponse>,
    slashing: Option<DamageResponse>,
    thunder: Option<DamageResponse>,
    // __all__: Vec<CatchAllDamageResponse>, // TODO: later...
}

impl DamageEffectorsRaw {
    pub fn construct(self, this: Weak<StatBlock>) -> DamageEffectors {
        let Self {
            acid,
            bludgeoning,
            cold,
            fire,
            force,
            lightning,
            necrotic,
            piercing,
            poison,
            psychic,
            radiant,
            slashing,
            thunder,
        } = self;

        fn do_thing<D: DamageType>(
            this: &Weak<StatBlock>,
            resp: Option<DamageResponse>,
        ) -> DamageP<D> {
            let mut proxy = DamageP::new(this.clone());

            if let Some(resp) = resp {
                // SAFETY: Same layout, only PhantomData<D>.
                let tmp = unsafe { (&mut proxy as *mut _ as *mut DamageP<()>).as_mut_unchecked() };
                (resp.0)(tmp)
            }

            proxy
        }

        let this = &this;
        DamageEffectors {
            acid: do_thing(this, acid),
            bludgeoning: do_thing(this, bludgeoning),
            cold: do_thing(this, cold),
            fire: do_thing(this, fire),
            force: do_thing(this, force),
            lightning: do_thing(this, lightning),
            necrotic: do_thing(this, necrotic),
            piercing: do_thing(this, piercing),
            poison: do_thing(this, poison),
            psychic: do_thing(this, psychic),
            radiant: do_thing(this, radiant),
            slashing: do_thing(this, slashing),
            thunder: do_thing(this, thunder),

            // TODO: This part.
            __all__: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "&str")]
pub struct DamageResponse(DamageResponseType);

pub struct UnrecognizedDamageResponse;

impl Display for UnrecognizedDamageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized damage response type -- try 'R' | 'resistance'; 'V' | 'vulnerability'; 'I' | 'immunity'")
    }
}

impl TryFrom<&str> for DamageResponse {
    type Error = UnrecognizedDamageResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DAMAGE_RESPONSES
            .get(value)
            .copied()
            .map(Self)
            .ok_or(UnrecognizedDamageResponse)
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "&str")]
pub enum ConditionResponse {
    Immunity,
}

pub struct UnrecognizedConditionResponse;

impl Display for UnrecognizedConditionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unrecognized condition response type -- try 'I' | 'immunity', or leave blank for none.")
    }
}

impl TryFrom<&str> for ConditionResponse {
    type Error = UnrecognizedConditionResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "I" => Ok(Self::Immunity),
            "immunity" => Ok(Self::Immunity),
            _ => Err(UnrecognizedConditionResponse),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct ConditionImmunitiesRaw {
    weak: Option<ConditionResponse>,
    blinded: Option<ConditionResponse>,
    charmed: Option<ConditionResponse>,
    deafened: Option<ConditionResponse>,
    exhaustion: Option<ConditionResponse>,
    frightened: Option<ConditionResponse>,
    grappled: Option<ConditionResponse>,
    incapacitated: Option<ConditionResponse>,
    invisible: Option<ConditionResponse>,
    paralyzed: Option<ConditionResponse>,
    petrified: Option<ConditionResponse>,
    poisoned: Option<ConditionResponse>,
    prone: Option<ConditionResponse>,
    restrained: Option<ConditionResponse>,
    stunned: Option<ConditionResponse>,
    unconscious: Option<ConditionResponse>,
}

impl ConditionImmunitiesRaw {
    pub fn construct(self, this: Weak<StatBlock>) -> ConditionImmunities {
        fn do_thing<C: Condition>(
            weak: &Weak<StatBlock>,
            opt: Option<ConditionResponse>,
        ) -> ConditionImmunityP<C> {
            let proxy = ConditionImmunityP::new(weak.clone());
            match opt {
                Some(ConditionResponse::Immunity) => proxy.insert(Immunity),
                None => (),
            }

            proxy
        }

        ConditionImmunities {
            weak: this.clone(),
            blinded: do_thing(&this, self.blinded),
            charmed: do_thing(&this, self.charmed),
            deafened: do_thing(&this, self.deafened),
            exhaustion: do_thing(&this, self.exhaustion),
            frightened: do_thing(&this, self.frightened),
            grappled: do_thing(&this, self.grappled),
            incapacitated: do_thing(&this, self.incapacitated),
            invisible: do_thing(&this, self.invisible),
            paralyzed: do_thing(&this, self.paralyzed),
            petrified: do_thing(&this, self.petrified),
            poisoned: do_thing(&this, self.poisoned),
            prone: do_thing(&this, self.prone),
            restrained: do_thing(&this, self.restrained),
            stunned: do_thing(&this, self.stunned),
            unconscious: do_thing(&this, self.unconscious),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct HealthRaw {
    max_hp: DExpr,

    #[serde(default)]
    hit_dice: HitDice,
}

impl HealthRaw {
    pub fn construct(self, this: Weak<StatBlock>) -> Health {
        let Self { max_hp, hit_dice } = self;

        Health {
            weak: this.clone(),
            hp: HP::new(
                this.clone(),
                Proxy::new(max_hp.result().max(1) as u32, this.clone()),
            ),
            conditions: RwLock::new(ConditionStatus::new(this)),
            hit_dice,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ACRaw {
    AC(DExpr),
}

impl ACRaw {
    fn construct(self, this: Weak<StatBlock>) -> AC {
        match self {
            ACRaw::AC(ac) => AC::with_base(
                this,
                ACPart {
                    source: Lifespan::Indefinite,
                    ac,
                },
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum CreatureTypeRaw {
    Player,
    Monster(MonsterRaw),
}

impl CreatureTypeRaw {
    pub fn construct(self, this: Weak<StatBlock>) -> CreatureType {
        match self {
            CreatureTypeRaw::Player => CreatureType::Player,
            CreatureTypeRaw::Monster(monster_raw) => {
                CreatureType::Monster(monster_raw.construct(this))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatBlockRaw {
    name: String,

    #[serde(rename = "type")]
    ty: self::CreatureTypeRaw,
    size: Size,
    scores: self::AbilityScoresRaw,

    #[serde(default)]
    skills: self::SkillsRaw,

    #[serde(default)]
    damage_effectors: self::DamageEffectorsRaw,

    #[serde(default)]
    condition_immunities: self::ConditionImmunitiesRaw,

    health: self::HealthRaw,

    ac: self::ACRaw,

    #[serde(default)]
    proficiency_bonus: Option<DExpr>,
}

impl StatBlockRaw {
    pub fn construct(self) -> Arc<StatBlock> {
        let Self {
            name,
            ty,
            size,
            scores,
            skills,
            damage_effectors: damages,
            health,
            proficiency_bonus,
            condition_immunities,
            ac,
        } = self;

        let is_monster = matches!(ty, CreatureTypeRaw::Monster(_));

        let s = Arc::new_cyclic(|this| StatBlock {
            name,
            ty: ty.construct(this.clone()),
            size,
            scores: scores.construct(this.clone()),
            modifiers: AbilityModifiers::new(this),
            skills: skills.construct(this.clone()),
            damage_effectors: damages.construct(this.clone()),
            health: health.construct(this.clone()),
            ac: ac.construct(this.clone()),
            condition_immunities: condition_immunities.construct(this.clone()),
            proficiency_bonus: {
                if !is_monster {
                    todo!("Players not supported yet!")
                }

                if let Some(prof) = proficiency_bonus {
                    ProficiencyBonus::fixed(this.clone(), prof)
                } else {
                    ProficiencyBonus::derived(this.clone())
                }
            },
            dead: RwLock::default(),
        });

        // TODO: Fix this evil hack.
        *s.health.hp.current.write().expect("Not poisoned.") = s.health.max_hp();

        s
    }
}

#[cfg(test)]
mod tests {
    use super::AbilityScoresRaw;

    #[test]
    fn parse_abilities() {
        let abilities: AbilityScoresRaw =
            toml::from_str("str = 2\ndex = 2\ncon = 3\nint = 23\nwis = 3\ncha = 3")
                .expect("Valid parse!");
        println!("{abilities:?}");
    }
}
