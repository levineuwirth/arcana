//! Proud Wildbonder — `{2}{R/G}{R/G}` 4/3 Human Warrior with Trample.
//! "Creatures you control with trample have 'You may have this creature
//! assign its combat damage as though it weren't blocked.'"
//!
//! Trample is a base keyword. The granted static — handing every
//! trample creature you control an assign-as-though-unblocked ability —
//! is a board-wide continuous ability not expressible as a
//! triggered/activated decomposition here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Proud Wildbonder");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/G}{R/G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static — grants trample creatures you control an
    // "assign combat damage as though unblocked" ability; no
    // board-wide ability-granting Effect available.
    reg.register(CardDefinition::new(name, chars))
}
