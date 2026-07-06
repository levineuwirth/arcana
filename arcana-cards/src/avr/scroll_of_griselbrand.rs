//! Scroll of Griselbrand — `{1}` artifact.
//! "{1}, Sacrifice this artifact: Target opponent discards a card. If you
//! control a Demon, that player loses 3 life." The Demon check is a
//! resolution-time board count (`script::count_matching`), matching the
//! oracle's resolution-time 'if you control'.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scroll of Griselbrand");
    let _demon = reg.interner_mut().intern("Demon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, Sacrifice this artifact: Target opponent \
                       discards a card. If you control a Demon, that \
                       player loses 3 life."
                    .into(),
                // GAP: 'target OPPONENT' — TargetFilter::Player has no
                // opponent constraint; any player is targetable.
                target_requirements: vec![TargetRequirement::target_opponent()],
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: discard_and_drain,
            },
        ),
    )
}

fn discard_and_drain(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![Effect::Discard {
        player: *p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }];
    let demons = script::subtype_filter(reg, "Demon")
        .controlled_by(ControllerConstraint::You);
    if script::count_matching(state, &demons, ctx.controller) > 0 {
        effects.push(Effect::LoseLife { player: *p, amount: 3 });
    }
    effects
}
