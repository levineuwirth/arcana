//! Focused Funambulist — `{1}{U}` 2/1 Human Performer with Flash.
//! "As this creature enters, you may put an art sticker on the empty
//! side of the balancing pole. When you do, ask a person outside the
//! game 'Does this look balanced?' If they say yes, you may tap or
//! untap target creature."
//!
//! Flash is a base keyword. The "art sticker" ETB ability is an Un-set
//! sticker / outside-the-game mechanic with no engine representation,
//! so it is GAP'd entirely.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Focused Funambulist");
    let human = reg.interner_mut().intern("Human");
    let performer = reg.interner_mut().intern("Performer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(performer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: "art sticker" ETB ability is an Un-set sticker /
    // outside-the-game mechanic with no engine representation. Omitted.
    reg.register(CardDefinition::new(name, chars))
}
