//! Ojutai Interceptor — `{3}{U}` 3/1 Creature — Bird Soldier.
//! Flying.
//! Megamorph {3}{U} — NOT in the usable keyword surface (the face-down
//! cast + turn-face-up-for-cost + counter mechanic is not modeled for this
//! card class). GAP: Megamorph omitted; only Flying is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ojutai Interceptor");
    let bird = reg.interner_mut().intern("Bird");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Megamorph {3}{U} — not in the usable keyword surface.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
