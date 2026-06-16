//! Mistcutter Hydra — `{X}{G}` 0/0 Hydra with Haste.
//!
//! This spell can't be countered.
//! Haste, protection from blue
//! This creature enters with X +1/+1 counters on it.
//!
//! Haste is wired. "Can't be countered", "protection from blue", and the
//! "enters with X +1/+1 counters" rider are not expressible with the
//! demonstrated API and are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mistcutter Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: "protection from blue" not in usable keyword surface.
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "This spell can't be countered" — no static cast-replacement primitive.
    // GAP: "enters with X +1/+1 counters" — no enters-with-X-counters rider.
    reg.register(CardDefinition::new(name, chars))
}
