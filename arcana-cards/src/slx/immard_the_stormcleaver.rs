//! Immard, the Stormcleaver — `{1}{U}{R}{W}` 4/4 Legendary Creature —
//! Human Soldier (R/U/W).
//!
//! "Whenever Immard, the Stormcleaver enters or attacks, put a charge counter
//!  on it or remove one from it. When you remove a counter this way, choose
//!  one —
//!  • Immard deals 4 damage to any target.
//!  • Immard gains lifelink and indestructible until end of turn."
//!
//! "Enters or attacks" decomposes into two triggers (`SelfEntersBattlefield`
//! and `SelfAttacks`). Both share the same payload, which is a player choice
//! between adding and removing a charge counter, with a reflexive modal
//! trigger ("when you remove a counter this way, choose one — …") on the
//! remove branch. There is no add-or-remove counter choice primitive and no
//! reflexive modal-trigger machinery, so both payloads are GAP'd; the trigger
//! shapes are recorded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Immard, the Stormcleaver");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: charge_counter_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: charge_counter_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn charge_counter_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put a charge counter on it or remove one from it" is a player
    // choice with no add-or-remove primitive, and "when you remove a counter
    // this way, choose one — [4 damage to any target] / [gain lifelink and
    // indestructible]" is a reflexive modal trigger with no supporting
    // machinery.
    Vec::new()
}
