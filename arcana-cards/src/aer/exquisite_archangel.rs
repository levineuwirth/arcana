//! Exquisite Archangel — `{5}{W}{W}` 5/5 Angel with Flying.
//!
//! "Flying" + "If you would lose the game, instead exile this creature
//! and your life total becomes equal to your starting life total."
//!
//! Decomposition:
//! * Flying → `keywords`.
//! * The "if you would lose the game, instead …" line is a
//!   replacement effect with no expressible primitive in this card
//!   class. GAP the static; the card emits a valid register fn with
//!   just the Flying keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exquisite Archangel");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: replacement static "If you would lose the game, instead exile this
    // creature and your life total becomes equal to your starting life total." —
    // a game-loss replacement effect has no expressible primitive in this class.

    reg.register(CardDefinition::new(name, chars))
}
