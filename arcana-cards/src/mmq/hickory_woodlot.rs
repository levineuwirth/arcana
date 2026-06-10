//! Hickory Woodlot — nonbasic land (Mercadian Masques, 1999).
//! "This land enters tapped with two depletion counters on it." and
//! "{T}, Remove a depletion counter from this land: Add {G}{G}. If
//! there are no depletion counters on this land, sacrifice it."
//! Enters-tapped is wired; GAP: "with two depletion counters on it"
//! has no EntersWithSpec variant (so in-engine the ability is never
//! activatable — the remove-counter cost gates on counters that are
//! never added), and the "if there are no depletion counters,
//! sacrifice it" rider has no constructible Effect::Conditional
//! condition.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hickory Woodlot");
    let depletion = reg.interner_mut().intern("depletion");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "enters tapped WITH TWO DEPLETION COUNTERS on it" —
            // EntersWithSpec has no enters-with-counters variant; only
            // the tapped half is modeled.
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove a depletion counter from this land: Add \
                       {G}{G}. If there are no depletion counters on this \
                       land, sacrifice it."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    remove_self_counter: Some((
                        CounterKind::Named(depletion),
                        1,
                    )),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_green,
            }),
    )
}

fn add_two_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If there are no depletion counters on this land, sacrifice
    // it" — Effect::Conditional's condition shape is not demonstrated
    // for source-counter checks; the sacrifice rider is omitted.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Green, ctx.source),
            ManaUnit::plain(ManaColor::Green, ctx.source),
        ],
    }]
}
