//! Boggart Shenanigans — `{2}{R}` Kindred Enchantment — Goblin.
//! "Whenever another Goblin you control is put into a graveyard from
//! the battlefield, you may have this enchantment deal 1 damage to
//! target player or planeswalker."
//!
//! GAPs: the Kindred card type is not a `TypeLine` const (Enchantment
//! only, Goblin kept as a subtype); "target player or planeswalker" is
//! narrowed to a player target; the "you may" is resolved as mandatory
//! (no free optional wrapper). "Another" is satisfied structurally —
//! the trigger filter requires a creature, which this enchantment is
//! not.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boggart Shenanigans");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        // GAP: type line is "Kindred Enchantment — Goblin"; the Kindred
        // card type has no TypeLine const, so only ENCHANTMENT is set.
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .with_subtypes_any(vec![goblin])
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: ping_for_goblin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // GAP: "target player or planeswalker" — planeswalker
                // targeting is not a TargetFilter variant; narrowed to a
                // player target.
                target_requirements: vec![TargetRequirement::target_player()],
            },
        ),
    )
}

/// "…you may have this enchantment deal 1 damage to target player or
/// planeswalker." (GAP: "you may" resolved as mandatory.)
fn ping_for_goblin(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
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
