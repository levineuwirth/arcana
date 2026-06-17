//! Maze Behemoth — `{5}{G}` 5/4 green Elemental with Trample.
//! "Multicolored creatures you control have trample." is a static
//! continuous anthem-style ability granting a keyword to a filtered
//! set of permanents — not a triggered/activated ability — and the
//! engine's available primitives here express only triggered /
//! activated abilities + this card's own keyword line, so the static
//! grant is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maze Behemoth");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "Multicolored creatures you control have trample." —
    // a filtered continuous keyword-grant anthem, not a triggered or
    // activated ability, and not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
