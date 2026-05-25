//! Riddlekeeper — `{2}{U}` 1/4 blue creature (Homunculus).
//! "Whenever a creature attacks you or a planeswalker you control, that
//! creature's controller mills two cards."
//!
//! GAP: trigger — "attacks you or a planeswalker you control" — no
//! TriggerCondition variant for creatures attacking a specific player;
//! using CreatureAttacks with Any controller as closest approximation.
//! GAP: "that creature's controller" — no accessor to retrieve the
//! controller of the attacking creature at trigger resolution.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riddlekeeper");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "attacks you or planeswalker you control"
                // not a supported condition; using CreatureAttacks Any as
                // closest approximation.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: on_creature_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that creature's controller mills" — attacker's controller
    // not accessible; milling the trigger controller as best effort.
    vec![Effect::Mill { player: trig.controller, count: 2 }]
}
