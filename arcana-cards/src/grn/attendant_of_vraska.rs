//! Attendant of Vraska — `{1}{B}{G}` 3/3 black-green Zombie Soldier.
//! "When this creature dies, if you control a Vraska planeswalker, you gain
//! life equal to this creature's power."
//!
//! Intervening-if "if you control a Vraska planeswalker" modeled via
//! `conditions::you_control_subtype` ("Vraska" is the planeswalker subtype).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Attendant of Vraska");
    let zombie = reg.interner_mut().intern("Zombie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: Some(iif_control_vraska),
            effect: on_dies_gain_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn iif_control_vraska(state: &GameState, _source: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_subtype(state, reg, you, "Vraska")
}

fn on_dies_gain_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::power_of(state, trig.dying_object().unwrap_or(trig.source)).max(0) as u32;
    vec![Effect::GainLife { player: trig.controller, amount: n }]
}
