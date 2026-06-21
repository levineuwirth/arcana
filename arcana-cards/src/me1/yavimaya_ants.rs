//! Yavimaya Ants — `{2}{G}{G}` 5/1 Creature — Insect.
//!
//! Trample, haste
//! * Cumulative upkeep {G}{G} — not an available keyword and not
//!   expressible as a trigger; GAP'd.
//!
//! GAP: Cumulative upkeep is not in the supported keyword surface and the
//! "add an age counter, then sacrifice unless you pay its upkeep cost for
//! each age counter" mechanic has no expressible primitive.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yavimaya Ants");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
