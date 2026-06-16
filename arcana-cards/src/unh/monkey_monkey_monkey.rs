//! Monkey Monkey Monkey — `{3}{G}` 1/1 green Monkey.
//!
//! Oracle:
//! * "As this creature enters, choose a letter." — an "as ~ enters" replacement
//!   choice (CR 614.12). Not a triggered/activated ability, and there is no
//!   demonstrated primitive for choosing a letter / storing that choice. GAP'd.
//! * "This creature gets +1/+1 for each nonland permanent whose name begins
//!   with the chosen letter." — a STATIC continuous self-pump whose amount
//!   depends on the chosen letter and on name-prefix matching. No trigger word,
//!   no cost, and no script helper for name-initial counts. GAP'd.
//!
//! Neither line is expressible with the demonstrated API, so only the bones are
//! emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "As this creature enters, choose a letter." — an as-enters replacement
// choice; no primitive to choose/store a letter.
// GAP: static "+1/+1 for each nonland permanent whose name begins with the
// chosen letter." — dynamic self-pump keyed on a name-initial count and the
// stored letter; no script helper for name-prefix matching, and a static
// continuous ability has no trigger/cost to wire.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monkey Monkey Monkey");
    let monkey = reg.interner_mut().intern("Monkey");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(monkey);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
