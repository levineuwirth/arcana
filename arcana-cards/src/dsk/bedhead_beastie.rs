//! Bedhead Beastie — `{4}{R}{R}` 5/6 Beast with Menace and Cycling.
//!
//! Oracle text:
//! * Menace — keyword, base characteristic.
//! * Mountaincycling {2} — per ENGINE CONVENTIONS, type/landcycling
//!   variants are not separately modeled; emit the generic
//!   `KeywordAbility::Cycling` with the printed `{2}` cost. The engine
//!   synthesizes the "{2}, discard this card: draw a card" activated
//!   ability. GAP (fidelity): the Mountain-search-instead-of-draw
//!   payoff of typecycling is not represented — plain Cycling draws.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bedhead Beastie");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Menace,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
