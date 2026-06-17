//! Deathleaper, Terror Weapon — `{2}{R}{G}` 3/3 Legendary Tyranid.
//!
//! Oracle:
//! * Flash
//! * Haste
//! * Flesh Hooks — Creatures you control that entered this turn have double
//!   strike.
//!
//! Flash and Haste are base characteristics. "Flesh Hooks" is a named
//! ability, not a KeywordAbility variant. Its body is a static continuous
//! ability (granting double strike to a dynamic set of creatures) with no
//! demonstrated primitive and is GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathleaper, Terror Weapon");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: "Flesh Hooks — Creatures you control that entered this turn have
    // double strike." — static continuous ability granting a keyword to a
    // dynamic set; no demonstrated primitive expresses it.
    reg.register(CardDefinition::new(name, chars))
}
