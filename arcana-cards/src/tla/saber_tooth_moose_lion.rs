//! Saber-Tooth Moose-Lion — `{4}{G}{G}` 7/7 Elk Cat with Reach.
//!
//! "Reach. Forestcycling {2}."
//!
//! Reach is a base keyword. Forestcycling is a typecycling variant —
//! per convention the generic Cycling with its printed cost is emitted;
//! the engine synthesizes the discard-to-draw activation. The
//! type-search ("Forest card") variant is not separately modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saber-Tooth Moose-Lion");
    let elk = reg.interner_mut().intern("Elk");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elk);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
