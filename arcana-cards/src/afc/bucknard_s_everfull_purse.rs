//! Bucknard's Everfull Purse — `{2}` colorless Artifact.
//! "{1}, {T}: Roll a d4 and create a number of Treasure tokens equal to the result.
//! The player to your right gains control of this artifact."
//! GAP: dice roll (d4) not in Effect catalog; Treasure count is variable. Control
//! transfer to "player to your right" approximated as first opponent.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bucknard's Everfull Purse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Roll a d4 and create that many Treasure tokens. The player to your right gains control of this artifact.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: roll_and_pass,
            }),
    )
}

fn roll_and_pass(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: d4 dice roll not in Effect catalog; Treasure token count is variable.
    // Partial: transfer control to first opponent (approximating "player to your right").
    let opp = script::opponents(state, ctx.controller)
        .into_iter()
        .next()
        .unwrap_or(ctx.controller);
    vec![Effect::ChangeControl { target: ctx.source, new_controller: opp }]
}
