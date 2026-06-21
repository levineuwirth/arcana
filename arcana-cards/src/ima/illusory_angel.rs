//! Illusory Angel — `{2}{U}` 4/4 Creature — Angel Illusion with Flying.
//!
//! Oracle:
//! * "Cast this spell only if you've cast another spell this turn." — a
//!   casting restriction; not expressible with the demonstrated API (no
//!   cast-legality gate for permanents). GAP'd.
//! * Flying.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illusory Angel");
    let angel = reg.interner_mut().intern("Angel");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: casting restriction "Cast this spell only if you've cast another
    // spell this turn" — no cast-legality gate for permanent spells.

    reg.register(CardDefinition::new(name, chars))
}
