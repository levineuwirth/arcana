//! Robaran Mercenaries — `{3}{W}` 3/4 Human Mercenary with Vigilance.
//! "This creature has all activated abilities of all legendary creatures
//! you control" is a static ability-granting effect that the demonstrated
//! API cannot express, so only the keyword is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Robaran Mercenaries");
    let human = reg.interner_mut().intern("Human");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "has all activated abilities of all legendary creatures
    // you control" — ability-granting continuous effect not expressible
    // with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
