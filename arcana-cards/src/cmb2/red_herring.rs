//! Red Herring — `{1}{R}` 2/2 Creature — Fish.
//! `{1}{U}: Exchange Red Herring from your hand with a permanent you control on the battlefield or a spell on the stack.`
//! GAP: "exchange from hand with permanent/spell" — no Effect for this.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Red Herring");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")), colors: ColorSet::red(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{1}{U}: Exchange this from your hand with a permanent you control.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{1}{U}").unwrap(), ..ActivationCost::default() }, target_requirements: Vec::new(), is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: exchange }))
}

fn exchange(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exchange from hand with permanent" — no Effect for this
    Vec::new()
}
