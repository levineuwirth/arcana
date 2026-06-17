//! Torpid Moloch — `{R}` 3/2 Lizard with Defender.
//! "Sacrifice three lands: This creature loses defender until end of turn."
//!
//! Defender is a base characteristic. The activated ability is GAP'd: there is
//! no documented effect to remove a single specific keyword (Defender) for a
//! turn — LoseAllAbilities is too broad (it would strip every ability, not just
//! Defender). An activated ability that paid the three-land sacrifice cost while
//! doing nothing would be a materially wrong card, so the whole ability is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Torpid Moloch");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: "Sacrifice three lands: This creature loses defender until end of
    // turn" — no documented effect to remove only the Defender keyword for a turn.
    reg.register(CardDefinition::new(name, chars))
}
