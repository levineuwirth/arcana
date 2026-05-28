//! Footlight Fiend — `{B/R}` 1/1 black/red Creature — Devil.
//! When this creature dies, it deals 1 damage to any target.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Footlight Fiend");
    let devil_sub = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{B/R}").expect("valid cost")), colors: ColorSet::black() | ColorSet::red(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef { id: 1, trigger_condition: TriggerCondition::SelfDies, intervening_if: None, effect: dies_ping, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime, target_requirements: vec![TargetRequirement::any_target()] }))
}

fn dies_ping(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    match target {
        TargetChoice::Object(id) => vec![Effect::DealDamage { target: DamageTarget::Object(*id), amount: 1, source: trig.source }],
        TargetChoice::Player(p) => vec![Effect::DealDamage { target: DamageTarget::Player(*p), amount: 1, source: trig.source }],
        _ => Vec::new(),
    }
}
