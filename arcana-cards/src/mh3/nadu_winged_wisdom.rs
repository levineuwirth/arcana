//! Nadu, Winged Wisdom — `{1}{G}{U}` 3/4 Legendary Bird Wizard.
//! Flying.
//! Creatures you control have "Whenever this creature becomes the target
//! of a spell or ability, reveal the top card of your library. If it's a
//! land card, put it onto the battlefield. Otherwise, put it into your
//! hand. This ability triggers only twice each turn."
//!
//! The ability-granting static (a continuous effect that grants every
//! creature you control a printed triggered ability, with a per-creature
//! twice-each-turn cap) has no expressible primitive — Flying is the
//! only emitted ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nadu, Winged Wisdom");
    let bird = reg.interner_mut().intern("Bird");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Creatures you control have '...becomes the target... reveal
    //      top card... twice each turn'." Static continuous grant of a
    //      printed triggered ability to all your creatures — no
    //      expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
