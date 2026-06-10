//! Nomad Stadium — nonbasic land (Odyssey, 2001).
//! "{T}: Add {W}. This land deals 1 damage to you." and
//! "Threshold — {W}, {T}, Sacrifice this land: You gain 4 life.
//! Activate only if there are seven or more cards in your graveyard."
//! The mana ability carries its self-damage rider (CR 605: it is still
//! a mana ability — no target, adds mana). The threshold activation's
//! "activate only if" legality gate is not expressible as an
//! ActivationCost field; it is enforced at resolution instead (GAP).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nomad Stadium");
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
                text: "{T}: Add {W}. This land deals 1 damage to you.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_white_with_sting,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Threshold — {W}, {T}, Sacrifice this land: You gain 4 \
                       life. Activate only if there are seven or more cards \
                       in your graveyard."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: threshold_gain_life,
            }),
    )
}

fn add_white_with_sting(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
        },
        Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
            source: ctx.source,
        },
    ]
}

fn threshold_gain_life(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Activate only if there are seven or more cards in your
    // graveyard" is an activation-legality gate; ActivationCost has no
    // condition field, so the threshold check is enforced at resolution.
    if script::graveyard_size(state, ctx.controller) < 7 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: ctx.controller, amount: 4 }]
}
