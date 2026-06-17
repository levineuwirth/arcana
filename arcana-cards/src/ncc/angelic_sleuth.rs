//! Angelic Sleuth — `{2}{W}` 2/3 Angel Advisor with Flying.
//! "Whenever another permanent you control leaves the battlefield, if it
//! had counters on it, investigate."
//!
//! Flying is a base keyword. Investigate maps to creating a Clue
//! commodity token. The "leaves the battlefield" trigger is modeled as a
//! ZoneChange to graveyard (the only single destination expressible; a
//! permanent could also leave to exile/hand/library — that breadth is a
//! GAP). The "if it had counters on it" intervening-if has no available
//! predicate (no "the leaving object had counters" condition), so it is
//! left None and GAP-noted.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angelic Sleuth");
    let angel = reg.interner_mut().intern("Angel");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger breadth — "leaves the battlefield" to any zone
            // is modeled as a ZoneChange to graveyard only.
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            // GAP: intervening-if "if it had counters on it" — no predicate
            // for "the leaving object had counters".
            intervening_if: None,
            effect: investigate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn investigate(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}
