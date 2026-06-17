//! The Knight of Land Drops — `{1}{R}` 1/1 Legendary Human Knight.
//! Partner with Knight (GAP). Static: Knights you control gain haste and a
//! granted end-step ability (GAP — pure continuous anthem-style static).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Knight of Land Drops");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    // GAP: keyword — "Partner with Knight" / Partner is not in the usable
    // KeywordAbility surface.
    // GAP: static — "Knights you control have haste and 'At the beginning of
    // your end step, if you didn't play a land this turn, you may discard a
    // card, then draw a card.'" is a continuous ability granting keywords +
    // a triggered ability to other permanents, not a triggered/activated
    // ability of this card.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
