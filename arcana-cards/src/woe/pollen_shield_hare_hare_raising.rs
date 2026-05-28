//! Pollen-Shield Hare // Hare Raising — `{1}{W}` // `{G}` white Adventure creature.
//! Creature: 2/2 Rabbit. "Creature tokens you control get +1/+1." (continuous effect)
//! Adventure (Hare Raising — Sorcery): Target creature you control gains vigilance and gets +X/+X until end of turn,
//!   where X = number of creatures you control.
//! GAP: "creature tokens get +1/+1" — continuous bonus not in catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pollen-Shield Hare");
    let adv_name = reg.interner_mut().intern("Hare Raising");
    let rabbit_sub = reg.interner_mut().intern("Rabbit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")), colors: ColorSet::white(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(2)), toughness: Some(PtValue::Fixed(2)), ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Target creature you control gains vigilance and gets +X/+X until end of turn, where X is the number of creatures you control.".into(), target_requirements: vec![TargetRequirement::target_creature()], modal: None, effect: hare_raising_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn hare_raising_resolve(state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let x = script::count_matching(state, &ObjectFilter::creature().controlled_by(ControllerConstraint::You), entry.controller) as i32;
    vec![Effect::Pump { target: *id, power: x, toughness: x, duration: Duration::EndOfTurn, keywords: vec![KeywordAbility::Vigilance] }]
}
