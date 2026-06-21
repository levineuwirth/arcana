//! Shatterskull Minotaur — `{4}{R}{R}` 5/4 red Minotaur Warrior.
//!
//! Oracle:
//! * "This spell costs {1} less to cast for each creature in your party."
//! * Haste
//!
//! Party-based cost reduction is a static casting-cost modifier; the
//! demonstrated API exposes no cost-reduction primitive, so only the
//! Haste keyword is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shatterskull Minotaur");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(warrior);

    // GAP: "This spell costs {1} less to cast for each creature in your
    // party." — no cost-reduction primitive in the demonstrated API.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
