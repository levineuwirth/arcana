//! Trueheart Duelist — `{1}{W}` 2/2 Human Warrior.
//! "This creature can block an additional creature each combat." +
//! Embalm {2}{W}. Both abilities are outside the demonstrated surface:
//! "blocks an additional creature" is a static with no Effect/keyword,
//! and Embalm is not in the usable KeywordAbility list.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trueheart Duelist");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "can block an additional creature each combat" — static with no
    // expressible Effect/keyword.
    // GAP: Embalm {2}{W} — not in the usable KeywordAbility surface.
    reg.register(CardDefinition::new(name, chars))
}
