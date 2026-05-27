//! Phyrexian Reaper — `{4}{B}` 3/3 black Creature — Phyrexian Zombie.
//! "Whenever this creature becomes blocked by a green creature, destroy that creature.
//! It can't be regenerated."
//! GAP: color filter on the blocking creature (green only) not checkable via trigger alone;
//! SelfBecomesBlocked fires regardless of blocker color. "Can't be regenerated" modifier not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Reaper");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: blocked_destroy_blocker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn blocked_destroy_blocker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: color filter (green) on the blocking creature not expressible without direct state access.
    // GAP: "can't be regenerated" modifier not modeled.
    // Best-effort: destroy the other_combatant (the blocker) unconditionally.
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: id }]
}
