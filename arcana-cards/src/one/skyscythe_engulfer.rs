//! Skyscythe Engulfer — `{5}{G}` 6/5 Phyrexian Beast with Reach and
//! Trample. The static "This creature can't be blocked by creatures
//! with flying" is a filtered block restriction with no expressible
//! primitive (CantBeBlocked is unconditional), so it is gapped.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyscythe Engulfer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "This creature can't be blocked by creatures with
    // flying" — no filtered can't-be-blocked-by primitive. Omitted.

    reg.register(CardDefinition::new(name, chars))
}
