//! Gathan Raiders — `{3}{R}{R}` 3/3 red Human Warrior.
//!
//! All of this card's non-bones text is unexpressible with the supported API:
//! * Hellbent — "+2/+2 as long as you have no cards in hand" is a pure
//!   continuous static (no trigger, no cost) — GAP.
//! * Morph—Discard a card — Morph is not in the supported keyword surface and
//!   the face-down cast / turn-face-up mechanic has no primitive — GAP.
//! Emitting bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "Hellbent — This creature gets +2/+2 as long as you have no cards in
// hand." — pure continuous static, not expressible as a trigger/activation.
// GAP: "Morph—Discard a card." — Morph keyword and the face-down cast / turn-
// face-up mechanic are not supported; keywords vec is empty.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gathan Raiders");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
