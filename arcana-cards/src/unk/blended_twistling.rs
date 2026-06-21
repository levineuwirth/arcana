//! Blended Twistling — `{2}{G/W}` 2/2 Creature — Shapeshifter with Changeling.
//!
//! Oracle:
//! * Changeling — base keyword (this creature is every creature type).
//! * This creature gets +X/+X, where X is your devotion to hybrid. — a STATIC
//!   self-pump; not a triggered/activated ability. It is also not expressible:
//!   `script::devotion` is keyed on a `ColorSet` (devotion to colors), and there
//!   is no "devotion to hybrid" measure, nor a continuous static-pump effect on
//!   this card class. GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blended Twistling");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    // GAP: static — "This creature gets +X/+X, where X is your devotion to
    // hybrid" (no devotion-to-hybrid measure; no continuous static self-pump).
    reg.register(CardDefinition::new(name, chars))
}
