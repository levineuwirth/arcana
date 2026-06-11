//! Aether Charge — `{4}{R}` enchantment.
//! "Whenever a Beast you control enters, you may have it deal 4 damage
//! to target opponent or planeswalker."
//!
//! A battlefield-bound `ZoneChange` trigger on Beasts you control; the
//! entering Beast deals the damage. GAPs: the cost-free "you may" gate
//! is not expressible (the damage fires whenever a target is chosen),
//! and "target opponent or planeswalker" has no exact `TargetFilter` —
//! `any_target()` is the documented closest declaration.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aether Charge");
    let beast = reg.interner_mut().intern("Beast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .with_subtypes_any(vec![beast])
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: beast_blast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target opponent or planeswalker" — no exact
                // TargetFilter (Player can't be opponent-and-planeswalker
                // combined); any_target() is the closest declaration.
                target_requirements: vec![TargetRequirement::any_target()],
            },
        ),
    )
}

/// "…you may have it deal 4 damage to target opponent or planeswalker."
fn beast_blast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(entered) = trig.entering_object() else {
        return Vec::new();
    };
    let Some(choice) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dmg_target = match choice {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(inner) => match inner {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    // GAP: the cost-free "you may" gate is not expressible; the damage
    // fires whenever a target was chosen.
    vec![Effect::DealDamage {
        target: dmg_target,
        amount: 4,
        source: entered,
    }]
}
