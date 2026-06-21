//! Grotag Bug-Catcher — `{1}{R}` 1/2 Goblin Warrior with Trample.
//! Whenever this creature attacks, it gets +1/+0 until end of turn for each
//! creature in your party (up to one each of Cleric, Rogue, Warrior, Wizard).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grotag Bug-Catcher");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![arcana_core::effects::KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: pump_for_party,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_for_party(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Party = up to one each of Cleric, Rogue, Warrior, Wizard you control.
    let mut party: u32 = 0;
    for sub in ["Cleric", "Rogue", "Warrior", "Wizard"] {
        let filter = script::subtype_filter(reg, sub)
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, trig.controller) > 0 {
            party += 1;
        }
    }
    if party == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: party as i32,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
