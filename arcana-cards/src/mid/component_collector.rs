//! Component Collector — `{2}{U}` 1/4 blue Homunculus.
//!
//! * "If it's neither day nor night, it becomes day as this creature
//!   enters." — modeled as a SelfEntersBattlefield trigger that sets
//!   day via `Effect::SetDayNight`. The "neither day nor night"
//!   precondition is not gateable (no day/night condition helper), so
//!   this fires unconditionally (minor fidelity GAP).
//! * "Whenever day becomes night or night becomes day, you may tap or
//!   untap target nonland permanent." — no day/night-change
//!   `TriggerCondition` variant exists; the whole ability is GAP'd
//!   (also note: the may-choose tap-OR-untap is not expressible).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::DayNight;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Component Collector");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: "Whenever day becomes night or night becomes day, you may tap
    // or untap target nonland permanent." — no day/night-change trigger
    // condition variant; whole ability omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                // GAP: "if it's neither day nor night" precondition is not
                // gateable; fires unconditionally.
                intervening_if: None,
                effect: etb_becomes_day,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_becomes_day(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::SetDayNight { value: DayNight::Day }]
}
