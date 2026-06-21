//! Cryptic Trilobite — `{X}{X}` 0/0 Trilobite.
//!
//! This creature enters with X +1/+1 counters on it.
//! Remove a +1/+1 counter from this creature: Add {C}{C}. Spend this mana only
//! to activate abilities.
//! {1}, {T}: Put a +1/+1 counter on this creature.
//!
//! The "enters with X +1/+1 counters" rider is GAP'd (no enters-with primitive
//! in the available activated/triggered surface). The two activated abilities
//! are implemented; the "spend only to activate abilities" rider on the mana is
//! a fidelity GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cryptic Trilobite");
    let trilobite = reg.interner_mut().intern("Trilobite");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(trilobite);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enters with X +1/+1 counters" — no enters-with primitive available.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP (fidelity): "Spend this mana only to activate abilities" rider.
                text: "Remove a +1/+1 counter from this creature: Add {C}{C}.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_cc,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter,
            }),
    )
}

fn add_cc(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 2],
    }]
}

fn add_counter(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
