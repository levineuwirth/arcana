//! Draconian Cylix — `{3}` artifact.
//! "{2}, {T}, Discard a card at random: Regenerate target creature."
//!
//! The discard-a-card cost uses `discard_other`; the "at random"
//! rider on the discard is GAP'd (the engine's discard_other cost is
//! a chosen card, not a random one).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Draconian Cylix");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {T}, Discard a card at random: Regenerate target creature.".into(),
            // GAP: "Discard a card AT RANDOM" — discard_other is a chosen
            // card; the random selection is not expressible as a cost.
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                tap: true,
                discard_other: Some(ObjectFilter::default()),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: regenerate_target,
        }),
    )
}

fn regenerate_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Regenerate { target: *id }]
}
