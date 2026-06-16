//! Palladia-Mors, the Ruiner — `{3}{R}{G}{W}` 6/6 Legendary Elder
//! Dragon with Flying, Vigilance, and Trample.
//!
//! "Palladia-Mors has hexproof if it hasn't dealt damage yet." — a
//! conditional self-granted-keyword static is not a demonstrated
//! primitive (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Palladia-Mors, the Ruiner");
    let elder = reg.interner_mut().intern("Elder");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    // GAP: "has hexproof if it hasn't dealt damage yet" — conditional
    // self-static keyword grant is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
