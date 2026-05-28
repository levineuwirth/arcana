//! Bramble Familiar // Fetch Quest — `{1}{G}` / `{5}{G}{G}` Adventure
//!
//! Creature: `{1}{G}` Creature — Elemental Raccoon (2/2)
//!   {T}: Add {G}. (GAP: activated mana ability deferred.)
//!   {1}{G}, {T}, Discard a card: Return to owner's hand. (GAP: activated deferred.)
//!
//! Adventure: `{5}{G}{G}` Sorcery — Fetch Quest
//!   Mill seven cards. Then put a creature, enchantment, or land card from
//!   among the milled cards onto the battlefield.
//!   (GAP: "from among milled cards" conditional ETB not expressible; Mill only.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bramble Familiar");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let raccoon_sub = reg.interner_mut().intern("Raccoon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental_sub);
    subtypes.0.insert(raccoon_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Fetch Quest");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Mill seven cards. Then put a creature, enchantment, or land card from among the milled cards onto the battlefield.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "from among milled cards" ETB not expressible
    vec![Effect::Mill { player: entry.controller, count: 7 }]
}
