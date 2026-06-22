//! Polar Kraken — `{8}{U}{U}{U}` 11/11 Kraken with Trample.
//! "This creature enters tapped."
//! "Cumulative upkeep—Sacrifice a land."
//!
//! Trample is wired. GAP: "enters tapped" has no enters-tapped primitive
//! for printed creature bones (same posture as Dread Wanderer). GAP:
//! "Cumulative upkeep—Sacrifice a land" — no CumulativeUpkeep keyword
//! and no age-counter / pay-per-counter upkeep machinery.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Polar Kraken");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(11)),
        toughness: Some(PtValue::Fixed(11)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "This creature enters tapped." — no enters-tapped primitive.
    // GAP: "Cumulative upkeep—Sacrifice a land." — no CumulativeUpkeep
    // keyword / age-counter upkeep machinery.

    reg.register(CardDefinition::new(name, chars))
}
