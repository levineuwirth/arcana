//! A-Kargan Warleader — `{1}{R}{W}` 3/3 Human Warrior with Ward {1}.
//! "Other Warriors you control get +1/+1 and have ward {1}." (static anthem — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Kargan Warleader");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: static "Other Warriors you control get +1/+1 and have ward {1}" — a pure
    // continuous anthem (no trigger/cost); not expressible as a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
