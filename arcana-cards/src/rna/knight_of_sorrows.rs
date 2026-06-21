//! Knight of Sorrows — `{4}{W}` 3/3 Human Knight.
//!
//! This creature can block an additional creature each combat. (static —
//!   GAP, no multi-block-grant primitive)
//! Afterlife 1 (When this creature dies, create a 1/1 white and black
//!   Spirit creature token with flying.)
//!
//! Afterlife 1 is the only expressible ability and is carried as a
//! keyword (the engine synthesizes the dies-trigger token). The
//! "can block an additional creature each combat" line is a pure static
//! continuous ability with no expressible primitive.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of Sorrows");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    // GAP: "can block an additional creature each combat" — static, no
    // multi-block-grant primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Afterlife(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
