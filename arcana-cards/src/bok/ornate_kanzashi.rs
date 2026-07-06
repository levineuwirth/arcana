//! Ornate Kanzashi — `{5}` artifact.
//! "{2}, {T}: Target opponent exiles the top card of their library.
//! You may play that card this turn."
//! Modeled with `Effect::ImpulseExile` over the targeted player's
//! library.
//! GAP fidelity: ImpulseExile grants the until-end-of-turn play
//! permission to the library's owner — the printed ability grants it
//! to YOU; and the 'target opponent' constraint is not expressible on
//! a player target.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ornate Kanzashi");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {T}: Target opponent exiles the top card of their library. You may play that card this turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_opponent()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: exile_their_top,
        }),
    )
}

fn exile_their_top(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: 'target opponent' constraint not expressible on a player
    // target. GAP fidelity: the play permission lands on the library's
    // owner, not on the Kanzashi's controller.
    vec![Effect::ImpulseExile { player: *p, count: 1 }]
}
