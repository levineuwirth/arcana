//! Henzie "Toolbox" Torre — `{B}{R}{G}` 3/3 Legendary Devil Rogue.
//!
//! Oracle:
//! * "Each creature spell you cast with mana value 4 or greater has
//!   blitz. The blitz cost is equal to its mana cost." — GAP: a static
//!   ability granting an alternative-cost cast modifier (Blitz);
//!   neither blitz nor cast-granting statics are expressible.
//! * "Blitz costs you pay cost {1} less for each time you've cast your
//!   commander from the command zone this game." — GAP: a static
//!   cost-reduction tied to commander-tax history; not expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Henzie \"Toolbox\" Torre");
    let devil = reg.interner_mut().intern("Devil");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: both lines are static cost/grant modifiers (Blitz-granting
    // and blitz cost reduction) — no triggered/activated form.
    reg.register(CardDefinition::new(name, chars))
}
