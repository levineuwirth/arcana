//! Riveteers Decoy — `{1}{G}` 3/1 Human Warrior.
//!
//! Oracle:
//! * This creature must be blocked if able. — a static combat-restriction
//!   (Lure-style). No `Effect` / ability shape expresses "must be blocked",
//!   so it is GAP'd.
//! * Blitz {3}{G} — an alternative casting cost. `Blitz` is not in the
//!   supported `KeywordAbility` surface and there is no alt-cost field, so
//!   it is GAP'd (`keywords: vec![]`).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riveteers Decoy");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: keyword — Blitz {3}{G} (alternative cast cost; not modeled).
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "This creature must be blocked if able." No Effect /
    // ability shape expresses a must-be-blocked combat restriction.
    reg.register(CardDefinition::new(name, chars))
}
