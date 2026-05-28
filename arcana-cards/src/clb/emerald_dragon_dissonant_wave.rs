//! Emerald Dragon // Dissonant Wave — `{4}{G}{G}` // `{2}{G}` green Adventure creature.
//! Creature: 4/4 Dragon. Flying, trample.
//! Adventure (Dissonant Wave — Instant): Counter target activated or triggered ability from a noncreature source.
//! GAP: "counter target activated or triggered ability from a noncreature source" — ability counter not in catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emerald Dragon");
    let adv_name = reg.interner_mut().intern("Dissonant Wave");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(4)), toughness: Some(PtValue::Fixed(4)), keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample], ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::INSTANT.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Counter target activated or triggered ability from a noncreature source.".into(), target_requirements: vec![], modal: None, effect: dissonant_wave_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn dissonant_wave_resolve(_state: &GameState, _entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "counter target activated or triggered ability from a noncreature source" — ability counter not in catalog
    Vec::new()
}
