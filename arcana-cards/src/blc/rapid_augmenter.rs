//! Rapid Augmenter — `{1}{U}{R}` 1/3 Otter Artificer.
//!
//! Rules text:
//! * Haste
//! * Whenever another creature you control with base power 1 enters, it gains
//!   haste until end of turn.
//! * Whenever another creature you control enters, if it wasn't cast, put a
//!   +1/+1 counter on this creature and this creature can't be blocked this turn.
//!
//! Haste and the first trigger (grant haste to a newly-entered base-power-1
//! creature you control) are faithful. The second trigger has an intervening-if
//! "if it wasn't cast" with no available predicate; firing the counter +
//! unblockable unconditionally would be wrong, so that effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Rapid Augmenter");
    let otter = reg.interner_mut().intern("Otter");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(otter);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_min_power(1)
                        .with_max_power(1),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: grant_haste_to_entering,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: counter_and_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grant_haste_to_entering(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.entering_object() else {
        return Vec::new();
    };
    // "another creature" — don't grant to this creature itself.
    if id == trig.source {
        return Vec::new();
    }
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Haste,
        duration: Duration::EndOfTurn,
    }]
}

fn counter_and_unblockable(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if it wasn't cast" intervening-if has no available predicate, so the
    //       payload (put a +1/+1 counter on this creature; it can't be blocked
    //       this turn) is omitted rather than firing on every creature you cast.
    Vec::new()
}
