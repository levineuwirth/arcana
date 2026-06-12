//! Vial Smasher, Gleeful Grenadier — `{B}{R}` 3/2 legendary black-red creature.
//! "Whenever another outlaw you control enters, Vial Smasher deals 1 damage to
//! target opponent."
//!
//! NOTE: "outlaw" is a creature type group (Assassin, Mercenary, Pirate, Rogue,
//! Warlock), modeled with a subtypes-any (OR) filter on the ZoneChange trigger;
//! "another" is enforced by skipping this creature's own entry.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vial Smasher, Gleeful Grenadier");
    let goblin = reg.interner_mut().intern("Goblin");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let assassin = reg.interner_mut().intern("Assassin");
    let pirate = reg.interner_mut().intern("Pirate");
    let rogue = reg.interner_mut().intern("Rogue");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(mercenary);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // "another outlaw you control" — Assassin/Mercenary/Pirate/
                // Rogue/Warlock (OR); self-entry is skipped in the effect fn.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .with_subtypes_any(vec![assassin, mercenary, pirate, rogue, warlock])
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: outlaw_enters_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn outlaw_enters_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "another outlaw" — skip this creature's own entry (it is a Mercenary).
    if trig.entering_object() == Some(trig.source) {
        return Vec::new();
    }
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(*p),
        amount: 1,
        source: trig.source,
    }]
}
