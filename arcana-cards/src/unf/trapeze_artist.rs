//! Trapeze Artist — `{W}` 2/1 Human Performer with Flying.
//! The "enters by being flipped from a height…" clause is an Un-set
//! physical-dexterity rule and is not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trapeze Artist");
    let human = reg.interner_mut().intern("Human");
    let performer = reg.interner_mut().intern("Performer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(performer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "enters by being flipped from a height of at least one foot … if it
    // lands face down or didn't turn over, return it to hand" — an Un-set
    // physical-coin-flip mechanic with no engine representation.
    reg.register(CardDefinition::new(name, chars))
}
