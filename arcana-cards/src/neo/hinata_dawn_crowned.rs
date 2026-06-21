//! Hinata, Dawn-Crowned — `{1}{U}{R}{W}` 4/4 Legendary Kirin Spirit.
//!
//! Oracle:
//! * Flying, trample.
//! * Spells you cast cost `{1}` less to cast for each target.
//! * Spells your opponents cast cost `{1}` more to cast for each target.
//!
//! Only the keyword line is expressible. Both cost-modification statics
//! (per-target cost reduction / increase) have no primitive and are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hinata, Dawn-Crowned");
    let kirin = reg.interner_mut().intern("Kirin");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kirin);
    subtypes.0.insert(spirit);

    // GAP: static — "Spells you cast cost {1} less for each target" (cost reduction).
    // GAP: static — "Spells your opponents cast cost {1} more for each target" (cost increase).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
