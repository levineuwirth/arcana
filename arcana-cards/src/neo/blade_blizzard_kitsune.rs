//! Blade-Blizzard Kitsune — `{2}{W}` 2/2 Fox Ninja with Double strike.
//!
//! GAP: "Ninjutsu {3}{W}" — Ninjutsu is not in the usable KeywordAbility
//! surface (the return-unblocked-attacker-and-put-this-onto-the-
//! battlefield-tapped-and-attacking cast mechanic is unmodeled here).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blade-Blizzard Kitsune");
    let fox = reg.interner_mut().intern("Fox");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
