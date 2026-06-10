//! Inferno Elemental — `{4}{R}{R}` 4/4 red creature. "Whenever this
//! creature blocks or becomes blocked by a creature, this creature deals
//! 3 damage to that creature."
//!
//! Wired with the bare `SelfBlocksOrBecomesBlocked` (a prior GAP claimed
//! no such variant existed — stale). "That creature" is recovered via
//! `script::blockers_of` + `script::attackers_blocked_by`; the two
//! directions are mutually exclusive per event, so the union is exactly
//! the paired creature(s), matching the oracle's per-creature trigger
//! (mirrors som/engulfing_slagwurm).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inferno Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: deal_3_to_blocking_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn deal_3_to_blocking_creature(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Per event only one direction is populated: blockers_of when this
    // creature is the blocked attacker, attackers_blocked_by when it blocks.
    let mut paired = script::blockers_of(state, trig.source);
    paired.extend(script::attackers_blocked_by(state, trig.source));
    paired
        .into_iter()
        .map(|id| Effect::DealDamage {
            target: DamageTarget::Object(id),
            amount: 3,
            source: trig.source,
        })
        .collect()
}
