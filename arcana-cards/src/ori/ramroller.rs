//! Ramroller — `{3}` 2/3 Artifact Creature — Juggernaut.
//! "This creature attacks each combat if able. (static combat requirement — GAP'd)
//!  This creature gets +2/+0 as long as you control another artifact. (static — GAP'd)"
//!
//! Both lines are pure statics (an attack requirement and a conditional
//! continuous buff) with no trigger/cost — neither is expressible as a
//! triggered or activated ability, so this is a bones-only card.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ramroller");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
