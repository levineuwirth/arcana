//! Legion Vanguard — `{1}{B}` 2/2 black Creature — Vampire Soldier.
//! {1}, Sacrifice another creature: This creature explores.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Legion Vanguard");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);
    subtypes.0.insert(soldier_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")), colors: ColorSet::black(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{1}, Sacrifice another creature: This creature explores.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{1}").unwrap(), sacrifice: true, ..ActivationCost::default() }, target_requirements: vec![], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: explore_self }))
}

fn explore_self(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Explore { player: ctx.controller, target: ctx.source }]
}
