//! Slick Imitator — `{1}{U}` 1/3 blue Ooze.
//!
//! * Changeling (keyword). Convoke and the "Start your engines!" / speed
//!   mechanic are not in the usable keyword surface; GAP'd.
//! * "Max speed — {1}, Sacrifice this creature: Copy target spell you
//!   control." — the max-speed activation gate is not expressible, so the
//!   whole activated ability is GAP'd (firing it without the speed gate would
//!   be materially wrong).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slick Imitator");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Convoke and Start your engines!/Max speed not expressible.
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    // GAP: "Max speed — {1}, Sacrifice: Copy target spell you control." — no
    // max-speed activation gate.
    reg.register(CardDefinition::new(name, chars))
}
