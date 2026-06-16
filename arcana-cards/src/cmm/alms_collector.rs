//! Alms Collector — `{3}{W}` 3/4 Cat Cleric with Flash.
//! "If an opponent would draw two or more cards, instead you and that
//! player each draw a card."
//!
//! Flash is a base keyword. The draw-replacement static is a replacement
//! effect, not a triggered or activated ability, and is not expressible
//! with the available primitives — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alms Collector");
    let cat = reg.interner_mut().intern("Cat");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    // GAP: "If an opponent would draw two or more cards, instead you and
    // that player each draw a card." — a draw-replacement static effect not
    // expressible with the available trigger/activated/effect primitives.
    reg.register(CardDefinition::new(name, chars))
}
