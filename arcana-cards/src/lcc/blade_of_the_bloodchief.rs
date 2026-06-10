//! Blade of the Bloodchief — `{1}` artifact — Equipment.
//! "Whenever a creature dies, put a +1/+1 counter on equipped
//! creature. If equipped creature is a Vampire, put two +1/+1
//! counters on it instead. Equip {1}." The dies-trigger condition is
//! wired (`ZoneChange` battlefield → graveyard), but the effect needs
//! the EQUIPPED creature's id, and no attached-to accessor exists —
//! the effect body is a GAP. The Equip {1} half is wired via
//! `with_equip`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blade of the Bloodchief");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{1}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: creature_died_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Whenever a creature dies: put a +1/+1 counter on equipped creature
/// (two if it's a Vampire).
// GAP: 'put a +1/+1 counter on equipped creature' — no accessor for the
// Equipment's attached_to creature from the trigger context (trig.source
// is the Equipment itself); the Vampire doubling is likewise unmodeled
fn creature_died_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}
