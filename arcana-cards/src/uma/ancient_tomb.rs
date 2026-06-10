//! Ancient Tomb — nonbasic land (Tempest).
//! "{T}: Add {C}{C}. This land deals 2 damage to you."
//!
//! One ability adding two colorless mana units plus a 2-damage rider
//! to the controller. Because the effect is not AddMana-only, it is
//! wired with `is_mana_ability: false` per the engine convention
//! (fidelity note: by CR 605.1a the printed ability is a mana ability;
//! the damage rider forces the stack path here).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ancient Tomb");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {C}{C}. This land deals 2 damage to you."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_mana_and_ping_self,
            },
        ),
    )
}

fn add_mana_and_ping_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); 2],
        },
        Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 2,
            source: ctx.source,
        },
    ]
}
