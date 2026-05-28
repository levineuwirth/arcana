//! Nezumi Shadow-Watcher — `{B}` 1/1 Creature — Rat Warrior.
//! Sacrifice this creature: Destroy target Ninja.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nezumi Shadow-Watcher");
    let rat = reg.interner_mut().intern("Rat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(warrior);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "Sacrifice this creature: Destroy target Ninja.".into(), cost: ActivationCost { sacrifice: true, ..ActivationCost::default() }, target_requirements: vec![TargetRequirement::target_creature()], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: destroy_ninja }))
}

fn destroy_ninja(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}
