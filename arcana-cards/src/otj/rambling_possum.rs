//! Rambling Possum — `{2}{G}` 3/3 Creature — Possum Mount.
//!
//! * Whenever this creature attacks while saddled, it gets +1/+2 until end of
//!   turn. Then you may return any number of creatures that saddled it this
//!   turn to their owner's hand.
//! * Saddle 1.
//!
//! GAP: Saddle is a keyword cost ability (tap creatures with total power N to
//!   become saddled) — not a `KeywordAbility` variant nor an expressible
//!   activation cost (no "tap creatures with total power ≥ N" cost), so it is
//!   omitted.
//! The attack trigger pumps this creature +1/+2 (the "it gets +1/+2" half).
//! GAP: the "while saddled" intervening-if has no exposed saddled-state
//!   predicate, and the "return any number of creatures that saddled it this
//!   turn" rider has no accessor for the saddling creatures — that half is
//!   omitted. The pump fires unconditionally as a faithful partial.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rambling Possum");
    let possum = reg.interner_mut().intern("Possum");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(possum);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
