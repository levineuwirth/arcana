//! The Capitoline Triad — `{10}` 7/7 Legendary Creature — God Artificer.
//!
//! Oracle text:
//! * Those Who Came Before — This spell costs {1} less to cast for each
//!   historic card in your graveyard. (Cost-reduction static — not
//!   expressible; GAP'd.)
//! * Exile any number of historic cards from your graveyard with total
//!   mana value 30 or greater: You get an emblem with "Creatures you
//!   control have base power and toughness 9/9."
//!
//! GAP: the cost-reduction keyword ("Those Who Came Before") is a casting
//! static with no expressible primitive.
//! GAP: the activated ability's cost ("Exile any number of historic cards
//! from your graveyard with total mana value 30 or greater") is not an
//! expressible ActivationCost shape, and the emblem payload (a continuous
//! base-P/T static granted via an emblem) is not expressible — the whole
//! ability is GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Capitoline Triad");
    let god = reg.interner_mut().intern("God");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    // GAP: cost-reduction static "Those Who Came Before".
    // GAP: emblem-granting activated ability — non-mana exile-from-graveyard
    // cost shape + continuous base-P/T emblem payload not expressible.
    reg.register(CardDefinition::new(name, chars))
}
