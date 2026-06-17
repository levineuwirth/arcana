//! Predatory Sludge — `{2}{B}` 3/3 black Ooze.
//! Menace.
//! As Predatory Sludge enters the battlefield, choose a permanent you don't
//! control. When the chosen permanent is put into a graveyard from the
//! battlefield, conjure a card named Predatory Sludge into your hand.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Predatory Sludge");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        // GAP: "As ~ enters, choose a permanent you don't control. When the
        // chosen permanent is put into a graveyard, conjure a card named
        // Predatory Sludge into your hand." Conjure is not modeled (Arena-only
        // mechanic; no Effect::Conjure), and there is no replacement-style
        // "as enters, choose a tracked permanent" hook with a linked delayed
        // ZoneChange watcher on that chosen object.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
