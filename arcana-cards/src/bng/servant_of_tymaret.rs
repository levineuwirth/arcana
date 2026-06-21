//! Servant of Tymaret — `{2}{B}` 1/3 Zombie.
//! Inspired — Whenever this creature becomes untapped, each opponent loses
//! 1 life and you gain that much life. `{2}{B}: Regenerate this creature.`
//!
//! The Inspired ("becomes untapped") trigger has no matching TriggerCondition
//! variant in the engine (only SelfBecomesTapped / BecomesTapped exist), so it
//! is GAP'd. The regenerate activated ability is fully expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Servant of Tymaret");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: trigger — "Inspired — Whenever this creature becomes untapped, each
    // opponent loses 1 life. You gain life equal to the life lost this way." No
    // SelfBecomesUntapped TriggerCondition variant exists (only SelfBecomesTapped).

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{B}: Regenerate this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: regenerate_self,
        }),
    )
}

fn regenerate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
