//! Mowu, Loyal Companion — `{3}{G}` 3/3 Legendary Creature — Dog.
//! Vigilance, trample.
//! "If one or more +1/+1 counters would be put on Mowu, that many plus
//! one +1/+1 counters are put on it instead." — a counter-doubling-style
//! replacement effect with no expressible Effect/static primitive in the
//! demonstrated API, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mowu, Loyal Companion");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "If one or more +1/+1 counters would be put on Mowu, that many
    // plus one are put on it instead." — a counter-placement replacement
    // effect; no Effect variant or static-replacement primitive in the
    // demonstrated API expresses it.
    reg.register(CardDefinition::new(name, chars))
}
