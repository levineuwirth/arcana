//! Figure of Fable — `{G/W}` 1/1 Creature — Kithkin (green/white).
//!
//! A level-up "leveler"-style figure with three activated abilities:
//! * "{G/W}: This creature becomes a Kithkin Scout with base power and
//!   toughness 2/3."
//! * "{1}{G/W}{G/W}: If this creature is a Scout, it becomes a Kithkin
//!   Soldier with base power and toughness 4/5."
//! * "{3}{G/W}{G/W}{G/W}: If this creature is a Soldier, it becomes a
//!   Kithkin Avatar with base power and toughness 7/8 and protection from
//!   each of your opponents."
//!
//! Each activation's load-bearing base-P/T change is wired via permanent
//! `SetBasePT` on self. GAPs: the subtype change (no AddSubtype effect), the
//! "If this is a Scout/Soldier" current-subtype gate, and the Avatar's
//! protection (Protection is not in the usable surface).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Figure of Fable");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G/W}: This creature becomes a Kithkin Scout with base power and toughness 2/3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G/W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_scout,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G/W}{G/W}: If this creature is a Scout, it becomes a Kithkin Soldier with base power and toughness 4/5.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G/W}{G/W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_soldier,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G/W}{G/W}{G/W}: If this creature is a Soldier, it becomes a Kithkin Avatar with base power and toughness 7/8 and protection from each of your opponents.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G/W}{G/W}{G/W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_avatar,
            }),
    )
}

fn become_scout(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: subtype change to "Kithkin Scout" (no AddSubtype effect).
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 2,
        toughness: 3,
        duration: Duration::Permanent,
    }]
}

fn become_soldier(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If this creature is a Scout" current-subtype gate and the
    // subtype change to "Kithkin Soldier" (no AddSubtype effect).
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 4,
        toughness: 5,
        duration: Duration::Permanent,
    }]
}

fn become_avatar(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If this creature is a Soldier" current-subtype gate, the subtype
    // change to "Kithkin Avatar", and protection from each opponent
    // (Protection not in the usable surface).
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 7,
        toughness: 8,
        duration: Duration::Permanent,
    }]
}
