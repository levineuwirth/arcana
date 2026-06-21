//! Sabin, Master Monk — `{4}{R}` 4/3 Legendary Human Noble Monk with
//! Double strike.
//! Blitz—{2}{R}{R}, Discard a card.
//! "You may cast this card from your graveyard using its blitz ability."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sabin, Master Monk");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    subtypes.0.insert(monk);
    // GAP: Blitz—{2}{R}{R}, Discard a card — Blitz is not a `KeywordAbility`
    // variant and the alternative-cost cast (with its haste + dies-draw +
    // end-step sacrifice riders) isn't expressible.
    // GAP: "You may cast this card from your graveyard using its blitz ability"
    // — graveyard-cast permission isn't expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
