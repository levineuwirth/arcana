//! Hearth Elemental // Stoke Genius — `{5}{R}` / `{1}{R}` Adventure
//!
//! Creature: `{5}{R}` Creature — Elemental (4/5)
//!   This spell costs {X} less to cast, where X is the number of instant or
//!   sorcery cards and cards with an Adventure in your graveyard.
//!   (GAP: cost-reduction based on graveyard count deferred.)
//!
//! Adventure: `{1}{R}` Sorcery — Stoke Genius
//!   Discard your hand, then draw two cards.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hearth Elemental");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Stoke Genius");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Discard your hand, then draw two cards.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Discard { player: entry.controller, count: 7, choice: DiscardChoice::ControllerChooses },
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
