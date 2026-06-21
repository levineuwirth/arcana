//! Neverwinter Hydra — `{X}{X}{G}{G}` 0/0 Creature — Hydra.
//!
//! Oracle:
//! * As this creature enters, roll X d6. It enters with a number of +1/+1
//!   counters on it equal to the total of those results.
//!   (GAP: depends on X from the cast cost and on rolling X dice — neither is
//!   available to a SelfEntersBattlefield resolver via the demonstrated
//!   `script::` helpers; the enter-with counters are not emitted.)
//! * Trample
//! * Ward {4}

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Neverwinter Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{4}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
