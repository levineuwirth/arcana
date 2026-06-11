//! Bone Sabres — `{2}{G}` Artifact — Equipment.
//! "Whenever equipped creature attacks, put four +1/+1 counters on it.
//! Equip {3}"
//!
//! `with_equip({3})` wires the Equip activation. GAP: "whenever EQUIPPED
//! creature attacks" has no trigger condition (no filter expresses
//! "attached to this"), and no accessor yields the attacking creature for
//! the counter placement — the trigger body is an honest no-op.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bone Sabres");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{3}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever equipped creature attacks" has no
                // condition; CreatureAttacks over creatures you control is
                // the closest (over-broad — fires for any of your
                // attackers).
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: grow_on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…put four +1/+1 counters on it."
fn grow_on_attack(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no accessor yields the attacking creature's ObjectId (needed
    // both to verify it is the equipped creature and to place the four
    // +1/+1 counters) — honest no-op.
    Vec::new()
}
