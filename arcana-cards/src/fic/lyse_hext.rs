//! Lyse Hext — `{1}{W}{U}` 2/2 Legendary Human Rebel Monk.
//! Prowess — NOT in the usable KeywordAbility surface (GAP'd).
//! "Noncreature spells you cast cost {1} less to cast." — a cost-reduction static,
//!  not expressible (GAP'd).
//! "As long as you've cast two or more noncreature spells this turn, Lyse Hext has
//!  double strike." — a conditional continuous static, not expressible (GAP'd).
//! No triggered or activated abilities are expressible; emit bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lyse Hext");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(monk);

    // GAP: Prowess keyword not in usable surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: noncreature-spell cost reduction static — not expressible.
    // GAP: conditional double-strike static (two+ noncreature spells this turn) — not expressible.
    reg.register(CardDefinition::new(name, chars))
}
