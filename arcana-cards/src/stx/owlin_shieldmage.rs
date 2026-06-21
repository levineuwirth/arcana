//! Owlin Shieldmage — `{3}{W}{B}` 3/3 Creature — Bird Warlock. White/black.
//! Flying.
//! Ward—Pay 3 life. (GAP'd — KeywordAbility::Ward carries a ManaCost only;
//! a non-mana ward cost like "Pay 3 life" is not expressible, so the Ward
//! keyword is omitted.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Owlin Shieldmage");
    let bird = reg.interner_mut().intern("Bird");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Ward—Pay 3 life — non-mana ward cost not expressible.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
