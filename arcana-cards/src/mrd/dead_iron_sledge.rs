//! Dead-Iron Sledge — `{1}` artifact — Equipment.
//! "Whenever equipped creature blocks or becomes blocked by a creature,
//! destroy both creatures. Equip {2}"
//!
//! GAP: trigger — "equipped creature blocks or becomes blocked" has no
//! equipped-creature trigger condition; wired on the closest variant,
//! `SelfBlocksOrBecomesBlocked`, which keys on this Equipment itself
//! (an Equipment never blocks, so the trigger is conservative).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dead-Iron Sledge");
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
            .with_equip(ManaCost::parse("{2}").expect("valid cost"))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: destroy_both,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…destroy both creatures."
fn destroy_both(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the equipped creature's id is not accessible from the
    // trigger (no attached_to accessor), so only the other combatant
    // can be destroyed — "both creatures" is half-modeled.
    let Some(other) = trig.other_combatant() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: other }]
}
