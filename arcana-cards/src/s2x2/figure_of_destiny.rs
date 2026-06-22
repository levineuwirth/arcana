//! Figure of Destiny — `{R/W}` 1/1 Kithkin.
//!
//! Oracle (three level-up activations):
//! * {R/W}: becomes a Kithkin Spirit with base power and toughness 2/2.
//! * {R/W}{R/W}{R/W}: if this creature is a Spirit, it becomes a Kithkin Spirit
//!   Warrior with base P/T 4/4.
//! * {R/W}{R/W}{R/W}{R/W}{R/W}{R/W}: if this creature is a Warrior, it becomes a
//!   Kithkin Spirit Warrior Avatar with base P/T 8/8, flying, and first strike.
//!
//! Expressible: the base power/toughness changes (Effect::SetBasePT permanent)
//! and the flying + first strike grants. GAP'd per ability: the subtype
//! additions (no add-subtype effect) and the "if it's a Spirit/Warrior"
//! activation gates (no subtype-precondition on activation).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Figure of Destiny");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R/W}: Figure of Destiny becomes a Kithkin Spirit with base power and toughness 2/2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R/W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_2_2,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R/W}{R/W}{R/W}: If Figure of Destiny is a Spirit, it becomes a Kithkin Spirit Warrior with base power and toughness 4/4.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R/W}{R/W}{R/W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_4_4,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}: If Figure of Destiny is a Warrior, it becomes a Kithkin Spirit Warrior Avatar with base power and toughness 8/8, flying, and first strike.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R/W}{R/W}{R/W}{R/W}{R/W}{R/W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_8_8,
            }),
    )
}

fn become_2_2(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: also adds the Spirit subtype (no add-subtype effect).
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::Permanent,
    }]
}

fn become_4_4(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if it's a Spirit" subtype gate (no subtype precondition); also adds
    // the Warrior subtype (no add-subtype effect).
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 4,
        toughness: 4,
        duration: Duration::Permanent,
    }]
}

fn become_8_8(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if it's a Warrior" subtype gate (no subtype precondition); also adds
    // the Avatar subtype (no add-subtype effect).
    vec![
        Effect::SetBasePT {
            target: ctx.source,
            power: 8,
            toughness: 8,
            duration: Duration::Permanent,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::Permanent,
        },
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::FirstStrike,
            duration: Duration::Permanent,
        },
    ]
}
