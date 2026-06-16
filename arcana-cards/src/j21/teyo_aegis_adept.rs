//! Teyo, Aegis Adept — `{2}{W}{W}` Legendary Planeswalker — Teyo,
//! starting loyalty 5. White.
//!
//! Oracle text:
//! * `+1`: Up to one target creature's base power perpetually becomes
//!   equal to its toughness. It perpetually gains "This creature can
//!   attack as though it didn't have defender."
//! * `−2`: Conjure a card named Lumbering Lightshield onto the
//!   battlefield.
//! * `−6`: You get an emblem with "At the beginning of your end step,
//!   return target white creature card from your graveyard to the
//!   battlefield. You gain life equal to its toughness."
//!
//! # Scope
//!
//! * The Scryfall keyword `Conjure` is not in the usable keyword
//!   surface — `keywords` left empty (default), gap noted.
//! * `+1` uses Alchemy "perpetually" modifications (perpetual base-P/T
//!   set + perpetual ability grant) — not expressible — GAP'd.
//! * `−2` is `Conjure` (Alchemy digital-only card creation) — no engine
//!   surface — GAP'd.
//! * `−6` grants a bespoke graveyard-reanimation emblem — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teyo, Aegis Adept");
    let teyo = reg.interner_mut().intern("Teyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(teyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature's base power \
                       perpetually becomes equal to its toughness. It \
                       perpetually gains \"This creature can attack as \
                       though it didn't have defender.\"".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Conjure a card named Lumbering Lightshield onto \
                       the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"At the beginning of your \
                       end step, return target white creature card from your \
                       graveyard to the battlefield. You gain life equal to \
                       its toughness.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six,
            }),
    )
}

/// `+1`: perpetual base-P/T set + perpetual attack-as-no-defender grant.
fn plus_one(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Alchemy "perpetually" modifications (perpetual base power set +
    // perpetual ability grant) are not expressible from the surface.
    Vec::new()
}

/// `−2`: Conjure Lumbering Lightshield.
fn minus_two(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Conjure (Alchemy digital card creation) has no engine surface.
    Vec::new()
}

/// `−6`: graveyard-reanimation emblem.
fn minus_six(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: bespoke end-step graveyard-reanimation emblem.
    Vec::new()
}
