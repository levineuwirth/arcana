//! Shinen of Life's Roar — `{1}{G}` 1/2 Spirit.
//! "All creatures able to block this creature do so." (Lure static)
//! "Channel — {2}{G}{G}, Discard this card: All creatures able to block target
//! creature this turn do so."
//!
//! GAP (static): "All creatures able to block this creature do so" (Lure) has
//! no Effect-catalog representation — omitted.
//! GAP (Channel effect): "All creatures able to block target creature this turn
//! do so" is the same unmodeled Lure effect. The Channel activated ability
//! (cast from hand, {2}{G}{G} + discard this card, targeting a creature) is
//! emitted with a GAP'd effect so its cost/zone/target shape is recorded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shinen of Life's Roar");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Channel — {2}{G}{G}, Discard this card: All creatures able to block target creature this turn do so.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G}{G}").expect("valid cost"),
                discard_self: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Hand,
            is_instant_speed: false,
            face_gate: None,
            effect: channel_lure,
        }),
    )
}

fn channel_lure(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "All creatures able to block target creature this turn do so"
    // (Lure) is not modeled in the effect catalog.
    Vec::new()
}
