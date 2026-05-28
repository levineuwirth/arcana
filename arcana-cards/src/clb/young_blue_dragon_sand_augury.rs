//! Young Blue Dragon // Sand Augury — `{4}{U}` // `{1}{U}` blue Adventure creature.
//! Creature: 3/3 Dragon. Flying.
//! Adventure (Sand Augury — Sorcery): Scry 1, then draw a card.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Young Blue Dragon");
    let adv_name = reg.interner_mut().intern("Sand Augury");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(3)), toughness: Some(PtValue::Fixed(3)), keywords: vec![KeywordAbility::Flying], ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")), colors: ColorSet::blue(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "Scry 1, then draw a card.".into(), target_requirements: vec![], modal: None, effect: sand_augury_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn sand_augury_resolve(_state: &GameState, entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Scry { player: entry.controller, count: 1 },
        Effect::DrawCards { player: entry.controller, count: 1 },
    ]
}
