//! Vodalian Mage — `{2}{U}` 1/1 blue Creature — Merfolk Wizard.
//! {U}, {T}: Counter target spell unless its controller pays {1}.
//! GAP: "counter unless controller pays {1}" — counterspell with mana exemption not in Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vodalian Mage");
    let merfolk_sub = reg.interner_mut().intern("Merfolk");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk_sub);
    subtypes.0.insert(wizard_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(1)), toughness: Some(PtValue::Fixed(1)), ..Default::default() };
    reg.register(CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef { text: "{U}, {T}: Counter target spell unless its controller pays {1}.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{U}").unwrap(), tap: true, ..ActivationCost::default() }, target_requirements: vec![TargetRequirement { filter: TargetFilter::Spell(ObjectFilter::new()), count: TargetCount::Exactly(1), controller: None }], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: true, face_gate: None, effect: counter_unless }))
}

fn counter_unless(_state: &GameState, _ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "counter unless controller pays {1}" — counterspell not in Effect catalog
    Vec::new()
}
