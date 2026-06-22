//! Streetwise Negotiator — `{1}{G}` 0/2 green Cat Citizen with Backup 1.
//!
//! "Backup 1 (When this creature enters, put a +1/+1 counter on target creature.
//!  If that's another creature, it gains the following ability until end of
//!  turn.)
//!  This creature assigns combat damage equal to its toughness rather than its
//!  power."
//!
//! "Backup" is an ability-word, not a `KeywordAbility` variant → keywords empty.
//! Backup's ETB places one +1/+1 counter on a target creature, which IS
//! expressible.
//!
//! The granted/static ability — "assigns combat damage equal to its toughness
//! rather than its power" — has no expressible Effect, so both the Backup grant
//! ("it gains the following ability until end of turn") and Streetwise's own
//! copy of that static are GAP'd.

// GAP (Backup grant + static): "assigns combat damage equal to its toughness
// rather than its power" — no Effect models a toughness-assigns-damage
// characteristic, so the granted-ability half of Backup and the creature's own
// static are not expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Streetwise Negotiator");
    let cat = reg.interner_mut().intern("Cat");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: backup_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn backup_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "it gains the following ability until end of turn" (assigns combat
    // damage = toughness) is not expressible; only the +1/+1 counter is placed.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
