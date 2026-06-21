//! Voice of All — `{2}{W}{W}` 2/2 white Angel.
//!
//! Oracle:
//! * Flying.
//! * "As this creature enters, choose a color." — an as-enters
//!   color-choice (a CR 614.12 replacement-style setup); no expressible
//!   "choose a color as it enters" hook. GAP'd.
//! * "This creature has protection from the chosen color." — a static
//!   protection ability; the `Protection` keyword has no `KeywordAbility`
//!   variant and there is no protection-from-color primitive. GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voice of All");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: as-enters — "choose a color" (no as-enters color-choice hook).
    // GAP: static — "protection from the chosen color" (no Protection
    // KeywordAbility variant / protection-from-color primitive).
    reg.register(CardDefinition::new(name, chars))
}
