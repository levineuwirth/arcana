//! Drownyard Behemoth — `{9}` 5/7 Creature — Eldrazi Crab with Flash.
//! Emerge {7}{U}. "This creature has hexproof as long as it entered this
//! turn."
//!
//! Flash is expressed as a keyword. Emerge is not in the usable keyword
//! surface, and the conditional self-hexproof static ("as long as it
//! entered this turn") is a continuous ability with no triggered/activated
//! decomposition — both are GAP'd below.

// GAP: Emerge {7}{U} — alternative cast cost not in the usable keyword
//      surface (no KeywordAbility::Emerge variant).
// GAP: "This creature has hexproof as long as it entered this turn." — a
//      condition-gated static continuous ability, not expressible as a
//      triggered or activated ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drownyard Behemoth");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let crab = reg.interner_mut().intern("Crab");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(crab);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
