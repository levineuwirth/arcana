//! Stormsurge Kraken — `{3}{U}{U}` 5/5 Kraken with Hexproof.
//!
//! GAP: "Lieutenant — As long as you control your commander, this creature
//! gets +2/+2 and has 'Whenever this creature becomes blocked, you may
//! draw two cards.'" is a commander-conditional STATIC continuous ability
//! (Lieutenant is not a supported keyword, and there is no primitive for a
//! "as long as you control your commander" conditional pump + granted
//! triggered ability). Bones + Hexproof only.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stormsurge Kraken");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Hexproof],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
