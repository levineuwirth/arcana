//! Chancellor of the Tangle — `{4}{G}{G}{G}` 6/7 Phyrexian Beast.
//!
//! * Vigilance, Reach.
//! * "You may reveal this card from your opening hand. If you do, at the
//!   beginning of your first main phase of the game, add {G}." — the
//!   opening-hand pre-game reveal mechanic is not expressible with the
//!   demonstrated API (no opening-hand reveal trigger / first-main-phase-
//!   of-the-game condition); GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chancellor of the Tangle");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Reach],
        // GAP: "reveal from your opening hand … add {G}" opening-hand
        // pre-game reveal mechanic — no expressible trigger condition.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
