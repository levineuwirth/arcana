//! Illusionary Informant — `{1}{U}` 1/3 blue Bird Illusion with Flying.
//!
//! * Draft this card face up.  (GAP'd — draft-matters ability.)
//! * During the draft, you may turn this card face down. If you do, look at the
//!   next card drafted by a player of your choice.  (GAP'd — draft-matters.)
//! * Flying.
//!
//! Only Flying is expressible. The two draft abilities operate during the
//! draft, not during a game, and have no in-game trigger/activation or
//! primitive to model.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illusionary Informant");
    let bird = reg.interner_mut().intern("Bird");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(illusion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Draft this card face up." and the during-draft face-down ability are
    // draft-matters mechanics with no in-game trigger/activation or primitive.
    reg.register(CardDefinition::new(name, chars))
}
