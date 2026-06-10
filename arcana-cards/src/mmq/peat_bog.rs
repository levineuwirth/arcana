//! Peat Bog — nonbasic land (Mercadian Masques).
//! "This land enters tapped with two depletion counters on it." and
//! "{T}, Remove a depletion counter from this land: Add {B}{B}. If
//! there are no depletion counters on this land, sacrifice it."
//!
//! Enters-tapped is wired via `EntersWithSpec::Tapped`; the two
//! entry depletion counters and the empty-sacrifice rider are GAP'd
//! (see notes). The mana ability removes one depletion counter as its
//! cost and adds {B}{B}.

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
    let name = reg.interner_mut().intern("Peat Bog");
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
            // GAP: "enters tapped WITH TWO DEPLETION COUNTERS on it" — no
            // EntersWithSpec variant places counters; only the tapped half
            // is modeled, so the land never actually carries the counters
            // that the mana ability's cost consumes.
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove a depletion counter from this land: Add {B}{B}. If there are no depletion counters on this land, sacrifice it.".into(),
                // GAP: "If there are no depletion counters on this land,
                // sacrifice it" — no post-resolution self-sacrifice
                // condition is expressible.
                cost: ActivationCost {
                    tap: true,
                    remove_self_counter: Some((CounterKind::Named(depletion), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_black,
            }),
    )
}

fn add_two_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Black, ctx.source),
            ManaUnit::plain(ManaColor::Black, ctx.source),
        ],
    }]
}
