//! Clay Golem — `{4}` 4/4 Artifact Creature — Golem.
//! "{6}, Roll a d8: Monstrosity X, where X is the result."
//! "Berserk — When this creature becomes monstrous, destroy target
//! permanent."
//!
//! Both abilities are GAP'd: there is no Monstrosity effect, no
//! die-roll cost, and no "becomes monstrous" trigger condition in the
//! demonstrated API. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clay Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "{6}, Roll a d8: Monstrosity X" — no die-roll cost and no
    // Monstrosity effect in the demonstrated API.
    // GAP: "When this creature becomes monstrous, destroy target
    // permanent." — no BecomesMonstrous trigger condition.
    reg.register(CardDefinition::new(name, chars))
}
