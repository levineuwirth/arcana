//! Dawnhand Dissident — `{B}` 1/2 Creature — Elf Warlock.
//!
//! * `{T}, Blight 1`: Surveil 1.
//! * `{T}, Blight 2`: Exile target card from a graveyard.
//! * During your turn, you may cast creature spells from among cards you own
//!   exiled with this creature by removing three counters from among creatures
//!   you control (alternative cost).
//!
//! Blight N ("put N −1/−1 counters on this creature" as part of the cost) is
//! modeled with the `add_self_counter` activation-cost field
//! (CounterKind::MinusOneMinusOne, N). The two tap activations are wired
//! faithfully (Surveil 1; exile a target card from a graveyard). The
//! card-exile linkage and the third static (alternative-cost cast of
//! exiled-with-this creature spells by removing counters from your creatures)
//! is an exile-association cast permission with no expressible primitive — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dawnhand Dissident");
    let elf = reg.interner_mut().intern("Elf");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Blight 1: Surveil 1.".into(),
                cost: ActivationCost {
                    tap: true,
                    add_self_counter: Some((CounterKind::MinusOneMinusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: surveil_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Blight 2: Exile target card from a graveyard.".into(),
                cost: ActivationCost {
                    tap: true,
                    add_self_counter: Some((CounterKind::MinusOneMinusOne, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::default(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_target_grave_card,
            }),
        // GAP: "During your turn, you may cast creature spells from among cards
        // you own exiled with this creature by removing three counters from
        // among creatures you control in addition to paying their other
        // costs." — exile-association tracking + alternative-cost cast
        // permission has no expressible primitive.
    )
}

/// `{T}, Blight 1`: Surveil 1.
fn surveil_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil { player: ctx.controller, count: 1 }]
}

/// `{T}, Blight 2`: Exile target card from a graveyard.
fn exile_target_grave_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ExileFromGraveyard { target: *id }]
}
