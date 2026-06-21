//! Igneous Pouncer — `{4}{B}{R}` 5/1 black/red Elemental with Haste.
//! "Swampcycling {2}, mountaincycling {2}" — the type-search cycling variants
//! aren't separately modeled; emitted as generic Cycling {2}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Igneous Pouncer");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: swampcycling/mountaincycling are typecycling variants — only the
        // generic Cycling cost is modeled, so the land-type search is lost. The
        // synthesized ability draws a card for {2}, discarding this card.
        keywords: vec![
            KeywordAbility::Haste,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
