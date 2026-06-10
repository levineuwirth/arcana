//! Sandstone Needle — nonbasic land (depletion land).
//! "This land enters tapped with two depletion counters on it." and "{T},
//! Remove a depletion counter from this land: Add {R}{R}. If there are no
//! depletion counters on this land, sacrifice it." Enters-tapped is wired;
//! entering WITH counters is a GAP (so the remove-counter cost is never
//! payable until the engine grows enters-with-counters), and the
//! no-counters-left self-sacrifice rider is likewise a GAP.

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
    let name = reg.interner_mut().intern("Sandstone Needle");
    let depletion = reg.interner_mut().intern("depletion");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: "enters tapped WITH TWO DEPLETION COUNTERS" — only the tapped
    // half is expressible (EntersWithSpec::Tapped); enters-with-counters is
    // not, so the remove-counter cost below is never satisfiable as wired.
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove a depletion counter from this land: Add {R}{R}. If there are no depletion counters on this land, sacrifice it.".into(),
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
                effect: add_double_red,
            }),
    )
}

fn add_double_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If there are no depletion counters on this land, sacrifice it" —
    // the source's counter count is not readable at resolution, so the
    // conditional self-sacrifice is omitted.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Red, ctx.source),
            ManaUnit::plain(ManaColor::Red, ctx.source),
        ],
    }]
}
