//! Inspirational Antelope — `{1}{G}` 1/3 green Antelope.
//! "Legacy — Before the game starts, choose a keyword or ability word
//!  and write it below. Spells with __________ you cast cost {1} less."
//!
//! Legacy is not an expressible keyword in this card class, and the
//! sole non-keyword text is a static cost-reduction tied to a
//! game-start choice — neither a triggered nor an activated ability.
//! Only the faithful bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inspirational Antelope");
    let antelope = reg.interner_mut().intern("Antelope");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(antelope);

    // GAP: keyword "Legacy" is not in the usable keyword surface; emit none.
    // GAP: static "Spells with [chosen word] you cast cost {1} less" — a
    //      game-start-choice cost-reduction static, not a triggered/activated
    //      ability and not expressible with the demonstrated API.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
