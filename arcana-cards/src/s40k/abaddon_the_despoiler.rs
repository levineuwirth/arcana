//! Abaddon the Despoiler — `{2}{U}{B}{R}` 5/5 Legendary Astartes Warrior with Trample.
//! "Mark of Chaos Ascendant — During your turn, spells you cast from your hand with
//! mana value X or less have cascade, where X is the total amount of life your
//! opponents have lost this turn."
//!
//! Only Trample is expressible. The Mark of Chaos Ascendant static (a continuous
//! cost/keyword-granting ability that confers cascade on other spells based on a
//! dynamic life-loss threshold) has no representation in the demonstrated API.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Abaddon the Despoiler");
    let astartes = reg.interner_mut().intern("Astartes");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        // GAP: "Mark of Chaos Ascendant" static — grants cascade to your hand-cast
        // spells with mana value <= (opponents' life lost this turn). No continuous
        // keyword-granting-to-other-spells primitive in the demonstrated API.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
