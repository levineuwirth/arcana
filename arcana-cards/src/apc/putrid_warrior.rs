//! Putrid Warrior — `{W}{B}` 2/2 Zombie Soldier Warrior.
//!
//! Oracle:
//! Whenever this creature deals damage, choose one —
//! • Each player loses 1 life.
//! • Each player gains 1 life.
//!
//! Decomposition: one triggered ability on "Whenever this creature deals
//! damage". The trigger condition is `DamageDealt` with a source filter that
//! matches this creature. The effect is a MODAL choice ("choose one — …"),
//! and triggered abilities have no modal mechanism in the demonstrated API
//! (only spell abilities carry `modal`). The choice between draining and
//! granting life to each player cannot be expressed, so the effect is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Putrid Warrior");
    let zombie = reg.interner_mut().intern("Zombie");
    let soldier = reg.interner_mut().intern("Soldier");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(soldier);
    subtypes.0.insert(warrior);

    let chars = arcana_core::objects::Characteristics {
        name,
        mana_cost: Some(arcana_core::mana::ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::AnyTarget,
                combat_only: false,
            },
            intervening_if: None,
            effect: damage_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn damage_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "choose one — each player loses 1 life / each player gains 1 life" is a
    // modal choice on a triggered ability; triggered abilities have no modal
    // mechanism in the demonstrated API (only spell abilities carry `modal`).
    Vec::new()
}
