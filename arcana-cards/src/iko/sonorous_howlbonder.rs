//! Sonorous Howlbonder — `{1}{B/R}{B/R}` 2/2 black/red Human Warrior.
//!
//! Menace.
//! Each creature you control with menace can't be blocked except by
//! three or more creatures. (GAP — board-wide block-count static.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sonorous Howlbonder");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/R}{B/R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "Each creature you control with menace can't be blocked except
    // by three or more creatures" — a board-wide static modifying the
    // required-blocker count is not expressible with the available API.

    reg.register(CardDefinition::new(name, chars))
}
