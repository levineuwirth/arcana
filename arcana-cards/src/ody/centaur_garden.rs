//! Centaur Garden — nonbasic land (Odyssey, 2001).
//! "{T}: Add {G}. This land deals 1 damage to you." and
//! "Threshold — {G}, {T}, Sacrifice this land: Target creature gets
//! +3/+3 until end of turn. Activate only if there are seven or more
//! cards in your graveyard."
//!
//! GAP: the self-damage rider on the mana ability is not expressible
//! (a mana ability's only result may be `Effect::AddMana`). GAP: the
//! threshold activation gate is not expressible as a precondition.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Centaur Garden");
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
                text: "{T}: Add {G}. This land deals 1 damage to you."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                // GAP: "This land deals 1 damage to you" rider — a mana
                // ability may only AddMana; the self-damage is omitted.
                effect: add_green_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Threshold — {G}, {T}, Sacrifice this land: Target \
                       creature gets +3/+3 until end of turn. Activate \
                       only if there are seven or more cards in your \
                       graveyard."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                // GAP: "Activate only if there are seven or more cards in
                // your graveyard" — activation preconditions are not
                // expressible.
                effect: threshold_pump,
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn threshold_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
