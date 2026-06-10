//! Remote Farm — nonbasic land. "This land enters tapped with two
//! depletion counters on it. {T}, Remove a depletion counter from this
//! land: Add {W}{W}. If there are no depletion counters on this land,
//! sacrifice it."
//!
//! Enters-tapped is wired; entering WITH two depletion counters is not
//! expressible (`EntersWithSpec::Tapped` only), so the
//! remove-a-counter cost will gate the ability closed — documented
//! gap. The no-counters sacrifice rider is also a gap.

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
    let name = reg.interner_mut().intern("Remote Farm");
    let depletion = reg.interner_mut().intern("depletion");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: "enters tapped WITH TWO DEPLETION COUNTERS" — EntersWithSpec
    // has no counters variant; only the tapped half is modeled, so the
    // mana ability below is never activatable (no counters to remove).
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Remove a depletion counter from this land: Add \
                       {W}{W}. If there are no depletion counters on this \
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
                effect: add_double_white,
            }),
    )
}

fn add_double_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If there are no depletion counters on this land, sacrifice
    // it" — no conditional sacrifice-self primitive demonstrated.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::White, ctx.source),
            ManaUnit::plain(ManaColor::White, ctx.source),
        ],
    }]
}
