//! Nishoba Brawler — `{1}{G}` */3 Cat Warrior with Trample.
//! Power is defined by Domain (the number of basic land types among
//! lands you control) — the CDA is GAP'd; power is printed as `*`.
//!
//! GAP (genuinely inexpressible): Domain counts the number of DISTINCT basic
//! land SUBTYPES (Plains/Island/Swamp/Mountain/Forest) among your lands, not a
//! single permanent count. `self_pt_from_match` is a single symmetric filter
//! count (can't count distinct subtypes), and `self_pt_cda`'s compute has no
//! `CardRegistry` so it can't resolve those subtype names to interned symbols.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: characteristic-defining ability "Domain — Nishoba Brawler's power
// is equal to the number of basic land types among lands you control" —
// a count of DISTINCT basic-land subtypes. self_pt_from_match is a single
// symmetric filter count and self_pt_cda's compute has no registry to
// resolve subtype names, so neither CDA constructor can express it; power
// is left as `*`.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nishoba Brawler");
    let cat = reg.interner_mut().intern("Cat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
