//! Bloodbraid Challenger — `{3}{R}{G}` 4/3 Elf Berserker with Haste.
//! Cascade and Escape are cast-time mechanics not in the usable
//! KeywordAbility surface for this card class.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodbraid Challenger");
    let elf = reg.interner_mut().intern("Elf");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: Cascade — cast trigger, not in the usable keyword surface for this class.
    // GAP: Escape—{3}{R}{G}, Exile three other cards from your graveyard — alternative
    //      cast cost, not in the usable keyword surface.
    reg.register(CardDefinition::new(name, chars))
}
