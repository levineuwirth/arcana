//! Lurking Green Dragon — `{3}{G}` 4/4 Dragon with Flying.
//! "This creature can't attack unless defending player controls a
//! creature with flying."
//!
//! Flying is a base characteristic. The conditional attack restriction
//! is a pure static ability with no trigger or cost, and there is no
//! Effect primitive that imposes a defending-player-conditional attack
//! restriction — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lurking Green Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "can't attack unless defending player controls a
    // creature with flying" — no expressible attack-restriction primitive.
    reg.register(CardDefinition::new(name, chars))
}
