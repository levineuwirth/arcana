//! Riddlekeeper — `{2}{U}` 1/4 blue creature (Homunculus).
//! "Whenever a creature attacks you or a planeswalker you control, that
//! creature's controller mills two cards."
//!
//! Wired via CreatureAttacks + trig.attacking_creature(): mills the
//! attacker's controller, gated on the attack defending Riddlekeeper's
//! controller or a planeswalker they control (read from the event).

use arcana_core::combat::DefendingEntity;
use arcana_core::effects::Effect;
use arcana_core::events::GameEvent;
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
                // Opponents' creatures attacking; the "attacks you or a
                // planeswalker you control" defender check happens in the
                // effect fn via the CreatureAttacks event.
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
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Mill THAT creature's controller, only when it attacks Riddlekeeper's
    // controller or a planeswalker they control.
    let Some(attacker) = trig.attacking_creature() else {
        return Vec::new();
    };
    let GameEvent::CreatureAttacks { defending, .. } = &trig.trigger_event else {
        return Vec::new();
    };
    let attacks_us = match defending {
        DefendingEntity::Player(p) => *p == trig.controller,
        DefendingEntity::Planeswalker(pw) => state
            .objects
            .get(*pw)
            .is_some_and(|o| o.controller == trig.controller),
        DefendingEntity::Battle(_) => false,
    };
    if !attacks_us {
        return Vec::new();
    }
    let Some(miller) = state.objects.get(attacker).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::Mill { player: miller, count: 2 }]
}
