//! Gods' Hall Guardian — `{5}{W}` 3/6 Cat with Vigilance.
//! Vigilance.
//! Foretell {3}{W}.
//!
//! Vigilance is a base characteristic. Foretell is NOT part of the usable
//! keyword surface for this card class, so the foretell alternative-cast
//! mechanic is GAP'd; the card is otherwise a vanilla vigilant body.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gods' Hall Guardian");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    // GAP: Foretell {3}{W} — Foretell is not part of the usable keyword surface
    // for this card class; the exile-face-down / later-cast mechanic is unmodeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
