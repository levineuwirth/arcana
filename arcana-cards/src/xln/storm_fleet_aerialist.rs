//! Storm Fleet Aerialist — `{1}{U}` 1/2 Human Pirate with Flying.
//! "Flying"
//! "Raid — This creature enters with a +1/+1 counter on it if you attacked this
//!  turn." (enters-with-counter replacement gated on a condition — GAP)
//!
//! Flying is the keyword line. Raid is not a usable KeywordAbility variant, and
//! there is no "enters with a counter if <condition>" replacement primitive, so
//! the Raid clause is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Storm Fleet Aerialist");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);

    // GAP: "Raid — This creature enters with a +1/+1 counter on it if you
    // attacked this turn." — an enters-with-counter replacement gated on a
    // turn-state condition; no expressible primitive (and Raid is not a usable
    // KeywordAbility variant).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
