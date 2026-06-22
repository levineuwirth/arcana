//! Chancellor of the Dross — `{4}{B}{B}{B}` 6/6 Phyrexian Vampire.
//!
//! * Flying, Lifelink.
//! * "You may reveal this card from your opening hand. If you do, at the
//!   beginning of the first upkeep, each opponent loses 3 life, then you
//!   gain life equal to the life lost this way." — the opening-hand
//!   pre-game reveal mechanic is not expressible with the demonstrated
//!   API (no opening-hand reveal trigger / first-upkeep-of-the-game
//!   condition); GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chancellor of the Dross");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        // GAP: "reveal from your opening hand … each opponent loses 3
        // life, then you gain that much" opening-hand pre-game reveal
        // mechanic — no expressible trigger condition.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
