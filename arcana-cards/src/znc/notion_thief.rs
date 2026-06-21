//! Notion Thief — `{2}{U}{B}` 3/1 Human Rogue with Flash.
//! "Flash
//!  If an opponent would draw a card except the first one they draw in
//!  each of their draw steps, instead that player skips that draw and you
//!  draw a card."
//!
//! Flash base keyword. The draw-replacement static (redirect each
//! opponent's extra draws to you) has no Effect/replacement primitive in
//! this card class — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Notion Thief");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: "If an opponent would draw a card except the first one ...
    // instead that player skips that draw and you draw a card" — a
    // draw-replacement static with no Effect/replacement primitive here.
    reg.register(CardDefinition::new(name, chars))
}
