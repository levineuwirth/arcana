//! Errant Ephemeron — `{6}{U}` 4/4 blue Illusion.
//!
//! * Flying.
//! * Suspend 4—{1}{U}.
//!
//! GAP (keyword line): Suspend is a cast-time alternative-cost mechanic
//! not in the usable `KeywordAbility` surface for this card class, so the
//! Suspend ability is omitted (`keywords: vec![KeywordAbility::Flying]`
//! only). The card is a vanilla Flying 4/4 in-engine; the suspend cast
//! path is unmodeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Errant Ephemeron");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
