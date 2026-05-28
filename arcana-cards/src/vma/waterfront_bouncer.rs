//! Waterfront Bouncer — `{1}{U}` 1/1 Creature — Merfolk Spellshaper.
//! `{U}, {T}, Discard a card: Return target creature to its owner's hand.`

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Waterfront Bouncer");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{U}, {T}, Discard a card: Return target creature to its owner's hand.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{U}").unwrap(), tap: true, ..ActivationCost::default() }, target_requirements: vec![TargetRequirement::target_creature()], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: bounce }))
}

fn bounce(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnToHand { target: *id }]
}
