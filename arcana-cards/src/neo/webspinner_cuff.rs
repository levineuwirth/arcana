//! Webspinner Cuff — `{2}{G}` 1/4 Artifact Creature — Equipment Spider, Reach.
//! Equipped creature gets +1/+4 and has reach (equip static — GAP).
//! Reconfigure {4} (GAP — not an expressible keyword/ability).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Webspinner Cuff");
    let equipment = reg.interner_mut().intern("Equipment");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment);
    subtypes.0.insert(spider);

    // GAP (static): "Equipped creature gets +1/+4 and has reach" — no equipped-
    //   creature continuous bonus primitive in this card class.
    // GAP (keyword/ability): "Reconfigure {4}" — not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
