//! Myr Quadropod — `{4}` 1/4 Artifact Creature — Myr.
//! `{3}: Switch this creature's power and toughness until end of turn.`
//! GAP: no Effect variant for "switch power/toughness" permanently or temporarily.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myr Quadropod");
    let myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")), colors: ColorSet::colorless(), types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE), subtypes, power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(4)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{3}: Switch this creature's power and toughness until end of turn.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{3}").unwrap(), ..ActivationCost::default() }, target_requirements: Vec::new(), is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: switch_pt }))
}

fn switch_pt(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no Effect variant for "switch power and toughness"
    Vec::new()
}
