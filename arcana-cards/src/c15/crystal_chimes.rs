//! Crystal Chimes — `{3}` artifact.
//! "{3}, {T}, Sacrifice this artifact: Return all enchantment cards
//! from your graveyard to your hand."
//!
//! GAP (fidelity): the mandatory "return ALL" is modeled with
//! `ChooseAnyNumberFromZone` (min-0/max-all pick) — returning every
//! enchantment is the max selection; the engine cannot force the full
//! set.

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crystal Chimes");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}, {T}, Sacrifice this artifact: Return all enchantment cards from your graveyard to your hand.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
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
            effect: recover_enchantments,
        }),
    )
}

/// "…Return all enchantment cards from your graveyard to your hand."
fn recover_enchantments(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ChooseAnyNumberFromZone {
        chooser: ctx.controller,
        zone: Zone::Graveyard(ctx.controller),
        filter: ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
        action: PickAction::ReturnToHand,
    }]
}
