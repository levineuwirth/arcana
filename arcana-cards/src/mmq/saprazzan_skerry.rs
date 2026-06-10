//! Saprazzan Skerry — nonbasic land (Mercadian Masques, 1999).
//! "This land enters tapped with two depletion counters on it." / "{T},
//! Remove a depletion counter from this land: Add {U}{U}. If there are no
//! depletion counters on this land, sacrifice it."

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
    let name = reg.interner_mut().intern("Saprazzan Skerry");
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
            // the tapped half is modeled, so the depletion counters are
            // never placed and the mana ability's remove-counter cost is
            // never payable.
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove a depletion counter from this land: Add \
                       {U}{U}. If there are no depletion counters on this \
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
                effect: add_two_blue,
            }),
    )
}

fn add_two_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if there are no depletion counters on this land, sacrifice
    // it" — the script surface has no counter-count accessor for a
    // resolution-time self-sacrifice check.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source); 2],
    }]
}
