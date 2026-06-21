//! Celestus Sanctifier — `{2}{W}` 3/2 Creature — Human Cleric.
//!
//! Oracle:
//! * "If it's neither day nor night, it becomes day as this creature
//!   enters." — modeled as an ETB trigger that sets day via
//!   `Effect::SetDayNight`. The "if it's neither day nor night"
//!   intervening-if has no demonstrated `conditions::` predicate, so it
//!   is GAP'd (intervening_if: None) and day is set unconditionally on
//!   entry (a fidelity gap when it is already night).
//! * "Whenever day becomes night or night becomes day, look at the top
//!   two cards of your library. Put one of them into your graveyard." —
//!   there is no day/night-change trigger condition, so this ability is
//!   GAP'd (omitted).

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
    let name = reg.interner_mut().intern("Celestus Sanctifier");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP (intervening-if): "if it's neither day nor night" has no
            // available conditions:: predicate; day is set unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_become_day,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_become_day(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::SetDayNight { value: DayNight::Day }]
}
