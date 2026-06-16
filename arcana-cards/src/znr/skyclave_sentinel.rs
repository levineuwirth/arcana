//! Skyclave Sentinel — `{3}` colorless Artifact Creature — Gargoyle
//! with Flying and Defender.
//!
//! "Kicker {4}. Flying, defender. If this creature was kicked, it enters
//! with two +1/+1 counters on it. As long as this creature has a +1/+1
//! counter on it, it can attack as though it didn't have defender."
//!
//! Flying and Defender are base keywords. Kicker is not a demonstrated
//! keyword variant; the kicked enter-with-counters replacement and the
//! "can attack as though it didn't have defender while it has a counter"
//! conditional static are both GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyclave Sentinel");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: Kicker {4} — not a demonstrated keyword variant.
    // GAP: "if kicked, enters with two +1/+1 counters" — ETB replacement.
    // GAP: "as long as it has a +1/+1 counter, can attack as though it
    //       didn't have defender" — conditional continuous static.
    reg.register(CardDefinition::new(name, chars))
}
