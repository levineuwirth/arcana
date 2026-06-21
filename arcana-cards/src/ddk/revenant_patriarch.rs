//! Revenant Patriarch — `{4}{B}` 4/3 Spirit.
//!
//! When this creature enters, if {W} was spent to cast it, target player skips
//! their next combat phase.
//! This creature can't block.
//!
//! The ETB trigger fires but is heavily GAP'd: the intervening-if "if {W} was
//! spent to cast it" has no mana-spent condition predicate, and "skips their
//! next combat phase" has no effect primitive. The "can't block" static
//! restriction is likewise GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Revenant Patriarch");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "This creature can't block" — static restriction, no primitive.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP (intervening-if): "if {W} was spent to cast it" — no mana-spent
            // condition predicate available.
            intervening_if: None,
            effect: skip_combat,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn skip_combat(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "skips their next combat phase" — no skip-phase effect primitive.
    Vec::new()
}
