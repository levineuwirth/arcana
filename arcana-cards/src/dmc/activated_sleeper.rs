//! Activated Sleeper — `{2}{B}` 0/0 Phyrexian Shapeshifter with Flash.
//!
//! Flash.
//! You may have this creature enter as a copy of any creature card in a
//! graveyard that was put there from the battlefield this turn, except it's a
//! Phyrexian in addition to its other types.
//!
//! The copy-as-enter replacement effect (cloning a creature card from a
//! graveyard, with the this-turn restriction and the added Phyrexian type)
//! has no expressible primitive and is GAP'd; only Flash is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Activated Sleeper");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: "enter as a copy of any creature card in a graveyard that was put
    // there from the battlefield this turn (also a Phyrexian)" — no
    // enter-as-copy-from-graveyard replacement primitive.
    reg.register(CardDefinition::new(name, chars))
}
