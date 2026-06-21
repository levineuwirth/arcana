//! Riot Piker — `{1}{R}` 2/1 Creature — Goblin Berserker.
//!
//! Oracle:
//! * First strike.
//! * This creature attacks each combat if able. (Static attack requirement —
//!   no trigger/cost; GAP'd.)
//!
//! Decomposition: First strike → `keywords`; the attack requirement is a pure
//! static with no demonstrated hook.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riot Piker");
    let goblin = reg.interner_mut().intern("Goblin");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: "This creature attacks each combat if able." — a static attack
    // requirement with no trigger word and no activation cost.

    reg.register(CardDefinition::new(name, chars))
}
