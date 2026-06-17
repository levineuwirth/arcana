//! Verge Rangers — `{2}{W}` 3/3 Human Scout Ranger.
//! First strike. "You may look at the top card of your library any time."
//! "As long as an opponent controls more lands than you, you may play
//! lands from the top of your library."
//!
//! First strike is a base keyword. The two static play-from-top-of-library
//! abilities are continuous permission grants with no triggered/activated
//! shape — not expressible here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Verge Rangers");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: static "you may look at the top card of your library any time" — no static-permission primitive.
    // GAP: static "you may play lands from the top of your library while an opponent controls more lands" — no static-permission primitive.
    reg.register(CardDefinition::new(name, chars))
}
