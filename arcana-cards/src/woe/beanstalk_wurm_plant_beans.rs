//! Beanstalk Wurm // Plant Beans — `{4}{G}` // `{1}{G}` green Adventure creature.
//! Creature: 5/4 Plant Wurm. Reach.
//! Adventure (Plant Beans — Sorcery): You may play an additional land this turn.
//! GAP: "play an additional land this turn" — additional-land-play not in catalog.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beanstalk Wurm");
    let adv_name = reg.interner_mut().intern("Plant Beans");
    let plant_sub = reg.interner_mut().intern("Plant");
    let wurm_sub = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant_sub);
    subtypes.0.insert(wurm_sub);
    let main_chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::CREATURE.into(), subtypes, supertypes: SupertypeSet::default(), power: Some(PtValue::Fixed(5)), toughness: Some(PtValue::Fixed(4)), keywords: vec![KeywordAbility::Reach], ..Default::default() };
    let adv_chars = Characteristics { name: adv_name, mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::SORCERY.into(), ..Default::default() };
    let adv_ability = SpellAbilityDef { text: "You may play an additional land this turn.".into(), target_requirements: vec![], modal: None, effect: plant_beans_resolve };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn plant_beans_resolve(_state: &GameState, _entry: &StackEntry, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "play an additional land this turn" — additional-land-play not in catalog
    Vec::new()
}
