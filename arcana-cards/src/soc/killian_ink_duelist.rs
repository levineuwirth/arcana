//! Killian, Ink Duelist — `{W}{B}` 2/2 Legendary Human Warlock.
//!
//! Oracle:
//! * Lifelink
//! * Menace
//! * Spells you cast that target a creature cost {2} less to cast.
//!
//! The two evergreen keywords are base characteristics. The cost-reduction
//! line is a static cost-modification ability with no demonstrated primitive
//! and is GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Killian, Ink Duelist");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink, KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Spells you cast that target a creature cost {2} less to cast." —
    // static cost-reduction ability; no demonstrated primitive expresses it.
    reg.register(CardDefinition::new(name, chars))
}
