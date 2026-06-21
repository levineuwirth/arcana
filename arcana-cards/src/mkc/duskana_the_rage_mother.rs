//! Duskana, the Rage Mother — `{2}{R}{G}{W}` 5/5 Legendary Bear.
//!
//! Oracle:
//! * When Duskana enters, draw a card for each creature you control with base
//!   power and toughness 2/2.
//! * Whenever a creature you control with base power and toughness 2/2 attacks,
//!   it gets +3/+3 until end of turn.
//!
//! GAP: both abilities key on "base power and toughness 2/2" — an exact
//! BASE-P/T filter. `ObjectFilter` exposes only `with_min_power` /
//! `with_max_power` / `with_max_toughness` (current values, no exact base-P/T
//! and no min-toughness), so the set cannot be faithfully expressed. Per the
//! dynamic-amount rule, the count and the attack-trigger membership are both
//! GAP'd whole (effects return `Vec::new()`) rather than approximated.

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
    let name = reg.interner_mut().intern("Duskana, the Rage Mother");
    let bear = reg.interner_mut().intern("Bear");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_for_each_2_2,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: attacker_pump_2_2,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_draw_for_each_2_2(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: count of creatures you control with BASE power and toughness 2/2 is
    // not an expressible filter (no exact base-P/T / min-toughness predicate).
    Vec::new()
}

fn attacker_pump_2_2(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the trigger should fire ONLY for an attacker with base P/T 2/2, which
    // is not an expressible condition on the trigger; firing-and-pumping every
    // attacker would be a materially wrong card, so the effect is GAP'd whole.
    Vec::new()
}
