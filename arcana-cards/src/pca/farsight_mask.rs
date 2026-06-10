//! Farsight Mask — `{5}` artifact (Mirrodin).
//! "Whenever a source an opponent controls deals damage to you, if
//! this artifact is untapped, you may draw a card."
//!
//! GAP: the intervening-if "if this artifact is untapped" has no
//! `conditions::` predicate (no source-untapped check) — the trigger
//! fires unconditionally. GAP: the "to you" restriction — the
//! `DamageDealt` target filter matches any player, not specifically
//! this artifact's controller. GAP: "you may" — the draw is modeled
//! as mandatory (no optional-trigger prompt).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Farsight Mask");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::Opponent),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                // GAP: "if this artifact is untapped" — no untapped-source
                // conditions:: predicate available.
                intervening_if: None,
                effect: draw_a_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may draw a card" — modeled as a mandatory draw.
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
