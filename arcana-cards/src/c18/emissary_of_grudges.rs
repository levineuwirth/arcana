//! Emissary of Grudges — `{5}{R}` 6/5 red Efreet.
//! Flying, haste.
//! As this creature enters, secretly choose an opponent.
//! Reveal the player you chose: Choose new targets for target spell or
//! ability if it's controlled by the chosen player and if it targets you or
//! a permanent you control. Activate only once.
//!
//! Flying and Haste are base keywords. The "as it enters, secretly choose an
//! opponent" replacement and the "reveal the chosen player: choose new
//! targets ... activate only once" ability are not expressible: there is no
//! secret-choice / reveal-as-cost primitive and no choose-new-targets Effect.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Emissary of Grudges");
    let efreet = reg.interner_mut().intern("Efreet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };
    // GAP: "as it enters, secretly choose an opponent" (no secret-choice
    // replacement primitive) and "Reveal the chosen player: choose new targets
    // for target spell or ability ... activate only once" (no reveal-as-cost
    // field, no choose-new-targets Effect).
    reg.register(CardDefinition::new(name, chars))
}
