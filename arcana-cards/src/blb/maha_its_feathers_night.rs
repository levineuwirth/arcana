//! Maha, Its Feathers Night — `{3}{B}{B}` 6/5 Legendary Elemental Bird
//! with Flying and Trample.
//! "Ward—Discard a card." (non-mana ward; not expressible)
//! "Creatures your opponents control have base toughness 1." (static)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maha, Its Feathers Night");
    let elemental = reg.interner_mut().intern("Elemental");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        // Ward—Discard a card is a non-mana ward cost, not expressible as a KeywordAbility.
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "Creatures your opponents control have base toughness 1" —
    // a continuous P/T-setting static, not a triggered or activated ability.
    reg.register(CardDefinition::new(name, chars))
}
