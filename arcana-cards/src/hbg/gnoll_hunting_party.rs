//! Gnoll Hunting Party — `{5}{R}` 4/4 Creature — Gnoll.
//!
//! All non-bone text is inexpressible with the available surface:
//! * "This spell costs {1} less to cast for each creature you attacked
//!   with this turn." — a cast cost reduction; no primitive; GAP'd.
//! * Double team — not in the usable keyword surface; GAP'd.
//! * "As long as it's your turn, Gnoll Hunting Party has first strike."
//!   — a conditional static; no triggered/activated hook; GAP'd.
//! Emitted as faithful bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnoll Hunting Party");
    let gnoll = reg.interner_mut().intern("Gnoll");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnoll);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: cost reduction ({1} less per creature attacked with this
    // turn); Double team keyword; conditional static first strike on
    // your turn — none expressible.

    reg.register(CardDefinition::new(name, chars))
}
