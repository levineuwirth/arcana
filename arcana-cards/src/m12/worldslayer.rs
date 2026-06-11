//! Worldslayer — `{5}` artifact — Equipment.
//! "Whenever equipped creature deals combat damage to a player,
//! destroy all permanents other than this Equipment."
//! "Equip {5}"
//!
//! The Equip half is real (`with_equip`). The triggered rider is a
//! GAP — `DamageDealt`'s source filter cannot express "the creature
//! this Equipment is attached to", so the trigger would over-fire on
//! EVERY creature's combat damage; the apocalypse effect is therefore
//! suppressed rather than wrongly fired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Worldslayer");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_equip(ManaCost::parse("{5}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever EQUIPPED creature deals
                // combat damage to a player": no ObjectFilter predicate
                // expresses "attached to this Equipment", so this fires
                // for any creature; the effect is suppressed to avoid a
                // wrongly-firing board wipe.
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: destroy_everything_else,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…destroy all permanents other than this Equipment."
fn destroy_everything_else(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the wired condition cannot be restricted to the equipped
    // creature (see register), so emitting the destroy-all here would
    // wipe the board on any creature's combat damage. Suppressed.
    Vec::new()
}
