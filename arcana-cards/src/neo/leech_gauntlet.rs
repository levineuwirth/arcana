//! Leech Gauntlet — `{1}{B}` 2/2 Artifact Creature — Equipment Leech with
//! Lifelink.
//!
//! "Equipped creature has lifelink." (static equip-grant — GAP'd)
//! "Reconfigure {4}" (the Reconfigure keyword/mechanic is not in the
//! documented keyword surface — GAP'd)
//!
//! Only the printed Lifelink keyword is expressible; the rest are documented
//! gaps.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP (static): "Equipped creature has lifelink" — equip-grant static with no
// documented Effect representation for this card class.
// GAP (keyword/mechanic): "Reconfigure {4}" — Reconfigure is outside the
// documented usable keyword surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leech Gauntlet");
    let equipment = reg.interner_mut().intern("Equipment");
    let leech = reg.interner_mut().intern("Leech");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(leech);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
