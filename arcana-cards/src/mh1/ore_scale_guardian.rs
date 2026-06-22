//! Ore-Scale Guardian — `{5}{R}{R}` 4/4 Dragon.
//! "This spell costs {1} less to cast for each land card in your
//!   graveyard." — GAP (a static cast-cost reduction; not a triggered/
//!   activated ability and no cost-reduction Effect).
//! Flying, haste.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ore-Scale Guardian");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    // GAP: "This spell costs {1} less to cast for each land card in your
    // graveyard." — static cast-cost reduction.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
