//! Nexus of Becoming — `{6}` artifact.
//! "At the beginning of combat on your turn, draw a card. Then you may
//! exile an artifact or creature card from your hand. If you do, create a
//! token that's a copy of the exiled card, except it's a 3/3 Golem
//! artifact creature in addition to its other types." The draw is wired;
//! the exile-from-hand choice plus modified card-copy token is a
//! documented gap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nexus of Becoming");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: draw_then_golemize,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn draw_then_golemize(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'you may exile an artifact or creature card from your hand. If
    // you do, create a token that's a copy of the exiled card, except it's
    // a 3/3 Golem artifact creature in addition to its other types' — no
    // primitive for an optional exile-from-hand pick feeding a modified
    // card-copy token (CopyPermanent copies battlefield objects only).
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
