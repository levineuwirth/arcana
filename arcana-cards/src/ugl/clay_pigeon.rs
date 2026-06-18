//! Clay Pigeon — `{3}` 1/1 Artifact Creature — Bird with Flying.
//! `{1}, Throw this creature into the air … : If you catch …, prevent all
//! damage a source of your choice would deal to you this turn and tap this
//! creature. Otherwise, sacrifice it.`
//!
//! The Flying keyword is fully modeled. The activated ability is an Un-set
//! physical-dexterity ability ("throw … catch with one hand") whose cost and
//! conditional resolution depend on a real-world coin-flip-of-skill that the
//! engine has no representation for — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: activated ability "{1}, Throw this creature into the air … : If you
// catch this creature …" — the cost is an Un-set physical-dexterity action and
// its resolution branches on a real-world skill outcome; neither the cost form
// nor the conditional is expressible with the demonstrated API.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clay Pigeon");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
