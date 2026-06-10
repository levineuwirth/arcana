//! Novijen, Heart of Progress — nonbasic land (Dissension, 2006).
//! "{T}: Add {C}." and "{G}{U}, {T}: Put a +1/+1 counter on each
//! creature that entered the battlefield this turn." The utility
//! activation's cost is wired; its effect is a GAP: there is no
//! sanctioned accessor for the set of creatures that ENTERED this turn
//! (no entered-this-turn ObjectFilter refinement or script:: id
//! enumeration), so the per-creature counter sweep cannot be built.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Novijen, Heart of Progress");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}{U}, {T}: Put a +1/+1 counter on each creature \
                       that entered the battlefield this turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{U}")
                        .expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counters_on_new_creatures,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn counters_on_new_creatures(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each creature that entered the battlefield this turn" —
    // no entered-this-turn filter refinement or script:: accessor
    // enumerates that set, so the counter sweep cannot be built.
    Vec::new()
}
