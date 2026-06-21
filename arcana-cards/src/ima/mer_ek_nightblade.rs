//! Mer-Ek Nightblade — `{3}{B}` 2/3 Creature — Orc Assassin.
//!
//! * Outlast {B} — "{B}, {T}: Put a +1/+1 counter on this creature. Outlast only
//!   as a sorcery." — modeled as an activated ability ({B} + tap, sorcery-speed)
//!   adding a +1/+1 counter to itself. (Outlast is not a usable keyword variant,
//!   so it is decomposed into its activated ability.)
//! * "Each creature you control with a +1/+1 counter on it has deathtouch." —
//!   a continuous static anthem. GAP: not expressible with the demonstrated API.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mer-Ek Nightblade");
    let orc = reg.interner_mut().intern("Orc");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}, {T}: Put a +1/+1 counter on this creature. Outlast only as a sorcery."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: outlast_counter,
        }),
    )
    // GAP: static "Each creature you control with a +1/+1 counter on it has
    // deathtouch" — continuous keyword-granting anthem, not expressible.
}

fn outlast_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
