//! Marhault Elsdragon — `{3}{R}{R}{G}` 4/6 Legendary Elf Warrior with Rampage 1.
//! Legends rare (1994); a red-green multicolor creature that grows larger
//! the more it is blocked.
//!
//! # Rules references
//!
//! * CR 702.23 — Rampage N. Whenever this creature becomes blocked, it gets
//!   +N/+N until end of turn for each creature blocking it beyond the first.
//!   Engine wiring is handled by the `Rampage(N)` parametrized keyword variant.
//!
//! Rampage is a fully implemented parametrized keyword; listing it in `keywords`
//! as `KeywordAbility::Rampage(1)` is sufficient — the runtime pipeline does the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marhault Elsdragon");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Rampage(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
