//! Countryside Crusher — `{1}{R}{R}` 3/3 red Giant Warrior.
//!
//! Oracle:
//! At the beginning of your upkeep, reveal the top card of your library. If
//! it's a land card, put it into your graveyard and repeat this process.
//! Whenever a land card is put into your graveyard from anywhere, put a +1/+1
//! counter on this creature.
//!
//! Decomposition:
//! * Upkeep trigger → "reveal the top card; if it's a land, mill it and
//!   repeat" is a reveal-until-nonland loop that LEAVES the found nonland on
//!   top of the library. `RevealUntil` only deposits the found card into hand
//!   or onto the battlefield (not "leave on top"), so this loop is not
//!   expressible — the upkeep trigger condition is registered with a GAP'd
//!   body.
//! * "Whenever a land card is put into your graveyard from anywhere, put a
//!   +1/+1 counter on this creature." → a `ZoneChange` trigger (land, any zone
//!   → graveyard) adding a +1/+1 counter to itself (fully wired).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Countryside Crusher");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_reveal_loop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: land_to_gy_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_reveal_loop(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top card; if it's a land, put it into your graveyard and
    // repeat" — RevealUntil cannot leave the found nonland on top of the
    // library, so this reveal-until-nonland loop is not expressible.
    Vec::new()
}

fn land_to_gy_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
