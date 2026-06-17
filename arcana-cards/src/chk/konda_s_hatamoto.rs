//! Konda's Hatamoto — `{1}{W}` 1/2 Human Samurai with Bushido 1.
//!
//! "As long as you control a legendary Samurai, this creature gets
//! +1/+2 and has vigilance" is a pure conditional STATIC continuous
//! ability (no trigger word, no cost), which is not a triggered/
//! activated ability — GAP'd. Bushido 1 is the only expressible piece.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Konda's Hatamoto");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Bushido(1)],
        ..Default::default()
    };

    // GAP (static): "As long as you control a legendary Samurai, this creature gets
    // +1/+2 and has vigilance" — conditional static continuous ability, not triggered/activated.
    reg.register(CardDefinition::new(name, chars))
}
