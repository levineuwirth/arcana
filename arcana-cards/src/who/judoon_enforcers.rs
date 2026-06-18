//! Judoon Enforcers — `{5}{R}{W}` 8/8 Alien Rhino Soldier with Trample.
//! Trample.
//! No more than one creature can attack you each combat.
//! Suspend 6—{1}{R}{W}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Judoon Enforcers");
    let alien = reg.interner_mut().intern("Alien");
    let rhino = reg.interner_mut().intern("Rhino");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(rhino);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "No more than one creature can attack you each combat." — static
    // attack-restriction, no Effect/static expressible.
    // GAP: Suspend is not in the usable keyword surface (no Suspend variant
    // available); the alternative-cast mechanic is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
