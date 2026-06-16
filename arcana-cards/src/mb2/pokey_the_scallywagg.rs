//! Pokey, the Scallywagg — `{U}{R}` 2/2 Legendary Brushwagg Pirate
//! with Menace.
//! The coin-flip / d20 substitution replacement abilities are GAP'd
//! (no replacement-effect primitive for swapping coin flips and die
//! rolls).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pokey, the Scallywagg");
    let brushwagg = reg.interner_mut().intern("Brushwagg");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(brushwagg);
    subtypes.0.insert(pirate);

    // GAP: "If you would flip a coin, you may instead roll a d20…"
    // and its mirror — coin/die substitution replacement effects are
    // not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
