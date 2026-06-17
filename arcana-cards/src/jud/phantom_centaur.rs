//! Phantom Centaur — `{2}{G}{G}` 2/0 Centaur Spirit.
//! "Protection from black."
//! "This creature enters with three +1/+1 counters on it."
//! "If damage would be dealt to this creature, prevent that damage. Remove a
//!  +1/+1 counter from this creature."
//!
//! Protection is not a usable KeywordAbility variant; the enters-with-counters
//! clause has no primitive here; and the damage-prevention/remove-counter
//! replacement is a bespoke static. All three are GAP'd — only bones emit.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phantom Centaur");
    let centaur = reg.interner_mut().intern("Centaur");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: keyword — "protection from black" is not a usable KeywordAbility.
    // GAP: static — "enters with three +1/+1 counters" (no enters-with form).
    // GAP: static — "prevent damage to this, remove a +1/+1 counter" is a
    // bespoke replacement effect with no Effect form here.
    reg.register(CardDefinition::new(name, chars))
}
