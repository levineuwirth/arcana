//! Rayne, Academy Chancellor — `{2}{U}` 1/1 blue Legendary Creature — Human Wizard.
//! "Whenever you or a permanent you control becomes the target of a spell or ability an
//! opponent controls, you may draw a card. You may draw an additional card if Rayne is
//! enchanted."
//!
//! # Notes
//! Two triggers: one for self being targeted, one for a permanent you control being targeted.
//! The "additional card if enchanted" clause is a conditional on a board state not expressible.
//! GAP: trigger for "a permanent you control becomes the target" — using SelfBecomesTarget
//! with caster: Opponent as closest match for the self-targeting trigger.
//! GAP: draw additional card if Rayne is enchanted — intervening_if not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rayne, Academy Chancellor");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: draw_on_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_on_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: draw an additional card if Rayne is enchanted — intervening_if condition not expressible.
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
