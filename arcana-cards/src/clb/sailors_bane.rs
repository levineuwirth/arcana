//! Sailors' Bane — `{7}{U}{U}` 7/7 Dragon Turtle with Ward {4}.
//!
//! The cost-reduction static ("costs {1} less for each instant/sorcery/
//! Adventure card you own in exile or graveyard") is a cost-modification
//! continuous ability with no expressible Effect/keyword primitive — GAP'd.
//! Ward {4} is a parametrized keyword and IS expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sailors' Bane");
    let dragon = reg.interner_mut().intern("Dragon");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(turtle);

    // GAP: "This spell costs {1} less to cast for each card you own in exile and
    // in your graveyard that's an instant card, a sorcery card, or a card that
    // has an Adventure." — a cost-reduction continuous ability; not expressible
    // with the available Effect / KeywordAbility surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{4}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
