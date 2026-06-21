//! Elderwood Scion — `{3}{G}{W}` 4/4 Creature — Elemental with Trample and
//! Lifelink.
//!
//! Oracle:
//! * Trample, lifelink.
//! * "Spells you cast that target this creature cost {2} less to cast." — a
//!   static cost-reduction; not expressible (GAP).
//! * "Spells your opponents cast that target this creature cost {2} more to
//!   cast." — a static cost-increase; not expressible (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elderwood Scion");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: static "Spells you cast that target this creature cost {2} less" —
    // no cost-reduction machinery.
    // GAP: static "Spells your opponents cast that target this creature cost
    // {2} more" — no cost-increase machinery.

    reg.register(CardDefinition::new(name, chars))
}
