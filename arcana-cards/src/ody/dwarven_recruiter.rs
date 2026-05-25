//! Dwarven Recruiter — `{2}{R}` 2/2 red Creature — Dwarf.
//! "When this creature enters, search your library for any number of Dwarf
//! cards, reveal them, then shuffle and put those cards on top in any order."
//! GAP: TutorToHand searches for one card; "any number" and "put on top in
//! any order" are not expressible. Using TutorToHand for a single Dwarf as
//! best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dwarven Recruiter");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_recruit_dwarves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_recruit_dwarves(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "any number" of cards and "put on top in any order" are not
    // expressible; using TutorToHand for one Dwarf as best effort
    let filter = script::subtype_filter(reg, "Dwarf");
    vec![Effect::TutorToHand { player: trig.controller, filter, reveal: true }]
}
