//! Salvage Titan — `{4}{B}{B}` 6/4 Artifact Creature — Golem.
//! "You may sacrifice three artifacts rather than pay this spell's mana
//!  cost." (alternative cost — not expressible.)
//! "Exile three artifact cards from your graveyard: Return this card from
//!  your graveyard to your hand." (exile-from-graveyard activation cost not
//!  expressible.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Salvage Titan");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "You may sacrifice three artifacts rather than pay this spell's mana
    //       cost." — alternative casting cost, no expressible surface.
    // GAP: "Exile three artifact cards from your graveyard: Return this card
    //       from your graveyard to your hand." — no exile-cards-from-graveyard
    //       activation cost field exists.
    reg.register(CardDefinition::new(name, chars))
}
