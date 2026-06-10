//! Bone Mask — `{4}` artifact (Mercadian Masques).
//! "{2}, {T}: The next time a source of your choice would deal damage to
//! you this turn, prevent that damage. Exile cards from the top of your
//! library equal to the damage prevented this way."
//!
//! Best-effort: prevention to you this turn via `Effect::PreventDamage`
//! (all sources — the source-of-your-choice narrowing is a documented
//! widening). GAP: the exile-equal-to-prevented rider.

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
    let name = reg.interner_mut().intern("Bone Mask");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {T}: The next time a source of your choice would deal damage to you this turn, prevent that damage. Exile cards from the top of your library equal to the damage prevented this way.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: prevent_to_you,
        }),
    )
}

fn prevent_to_you(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "a source of your choice" (single-source narrowing) and "Exile
    // cards from the top of your library equal to the damage prevented this
    // way" are not expressible; modeled as preventing damage to you until
    // end of turn.
    vec![Effect::PreventDamage {
        target: DamageTarget::Player(ctx.controller),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
