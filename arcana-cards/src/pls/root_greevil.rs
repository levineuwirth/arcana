//! Root Greevil — `{3}{G}` 2/3 Creature — Beast.
//! `{2}{G}, {T}, Sacrifice this creature: Destroy all enchantments of the color of your choice.`
//! GAP: "destroy all enchantments of chosen color" not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Root Greevil");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(3)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{2}{G}, {T}, Sacrifice this creature: Destroy all enchantments of chosen color.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{2}{G}").unwrap(), tap: true, sacrifice: true, ..ActivationCost::default() }, target_requirements: Vec::new(), is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: destroy_enchantments }))
}

fn destroy_enchantments(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "destroy all enchantments of chosen color" — color choice + type destruction not in catalog
    Vec::new()
}
