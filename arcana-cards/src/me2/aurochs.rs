//! Aurochs — `{3}{G}` 2/3 Aurochs with Trample.
//! "Whenever this creature attacks, it gets +1/+0 until end of turn for
//! each other attacking Aurochs."
//!
//! Trample is a base keyword. The attack trigger is wired with the
//! correct `SelfAttacks` condition, but its body is a per-ability GAP:
//! the pump amount is "+1/+0 for each OTHER ATTACKING Aurochs", a
//! dynamic count restricted to creatures currently attacking. The
//! `script::*` helpers count battlefield permanents by an `ObjectFilter`
//! but expose no combat-status ("attacking") predicate, so the count of
//! attacking Aurochs cannot be computed without inventing API. Per the
//! dynamic-amount rule, emitting a fixed pump would be a materially
//! wrong card, so the whole effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Aurochs");
    let aurochs = reg.interner_mut().intern("Aurochs");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aurochs);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_pump_per_attacking_aurochs,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_pump_per_attacking_aurochs(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+1/+0 until end of turn for each other ATTACKING Aurochs."
    // The amount is dynamic on a combat-status-restricted count
    // (Aurochs that are currently attacking); the script:: helpers have
    // no "attacking" predicate, so this count is not computable. A fixed
    // pump would be materially wrong, so the whole effect is GAP'd.
    Vec::new()
}
