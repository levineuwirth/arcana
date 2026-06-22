//! Duskrider Peregrine — `{5}{W}` 3/3 Creature — Bird with Flying.
//! "Flying, protection from black
//!  Suspend 3—{1}{W}"
//!
//! Only Flying is expressible. Protection (from black) is not in the usable keyword
//! surface, and Suspend is not a modeled keyword — both are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duskrider Peregrine");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "protection from black" — Protection is not in the usable keyword
        // surface. "Suspend 3—{1}{W}" — Suspend is not a modeled keyword.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
