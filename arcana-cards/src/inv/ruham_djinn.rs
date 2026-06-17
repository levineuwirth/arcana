//! Ruham Djinn — `{5}{W}` 5/5 Djinn with First strike.
//! "This creature gets -2/-2 as long as white is the most common color among all
//! permanents or is tied for most common."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ruham Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: conditional continuous static "gets -2/-2 as long as white is the most
    // common color among all permanents (or tied)" — a board-state-dependent
    // continuous self-debuff, not a triggered/activated ability, and no script
    // helper computes most-common-color. Not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
