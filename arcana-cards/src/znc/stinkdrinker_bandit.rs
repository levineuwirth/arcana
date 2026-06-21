//! Stinkdrinker Bandit — `{3}{B}` 2/1 Goblin Rogue.
//! "Prowl {1}{B}
//!  Whenever a Rogue you control attacks and isn't blocked, it gets
//!  +2/+1 until end of turn."
//!
//! Prowl is an alternative-cast keyword not in the usable set — GAP'd.
//! The attack trigger is wired via `CreatureAttacks { filter: Rogue you
//! control }`, pumping the attacker +2/+1. There is no filtered
//! "attacks and isn't blocked" variant, so the unblocked restriction is
//! dropped (a documented over-fire).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Stinkdrinker Bandit");
    let goblin = reg.interner_mut().intern("Goblin");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Prowl {1}{B} — alternative-cast keyword not in usable set.
        ..Default::default()
    };

    let rogue_sym = reg.interner_mut().intern("Rogue");

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_subtype_sym(rogue_sym),
            },
            // GAP: "and isn't blocked" restriction — no filtered
            // attacks-unblocked variant; over-fires on blocked attackers.
            intervening_if: None,
            effect: pump_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn pump_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: id,
        power: 2,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
