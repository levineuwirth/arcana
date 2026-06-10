//! Pentagram of the Ages — `{4}` artifact.
//! "{4}, {T}: The next time a source of your choice would deal damage to
//! you this turn, prevent that damage." Modeled with a this-turn
//! prevention shield on the controller; the one-shot / chosen-source
//! granularity is a documented fidelity GAP.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pentagram of the Ages");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: The next time a source of your choice \
                       would deal damage to you this turn, prevent that \
                       damage."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_next_damage_to_you,
            },
        ),
    )
}

fn prevent_next_damage_to_you(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the card prevents only the NEXT damage from ONE chosen source;
    // PreventDamage with amount: None shields the player from all damage
    // this turn (over-broad — no chosen-source / one-shot prevention).
    vec![Effect::PreventDamage {
        target: DamageTarget::Player(ctx.controller),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
