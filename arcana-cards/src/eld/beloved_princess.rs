//! Beloved Princess — `{W}` 1/1 Creature — Human Noble with Lifelink.
//! "This creature can't be blocked by creatures with power 3 or greater."
//!
//! The power-filtered evasion static is GAP'd (not expressible from the demonstrated
//! API — `CantBeBlocked` is unconditional, with no blocker-power filter).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beloved Princess");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: static "This creature can't be blocked by creatures with power 3 or
    // greater." — a blocker-power-filtered evasion static is not expressible.

    reg.register(CardDefinition::new(name, chars))
}
