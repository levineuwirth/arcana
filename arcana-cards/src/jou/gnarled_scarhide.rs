//! Gnarled Scarhide — `{B}` 2/1 Enchantment Creature — Minotaur.
//! Bestow {3}{B}.
//! This creature can't block.
//! Enchanted creature gets +2/+1 and can't block.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnarled Scarhide");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Bestow {3}{B}" is not in the supported KeywordAbility surface
        // (Bestow casts this as an Aura; the alternate-cost/attach mechanic is
        // not expressible here).
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "This creature can't block." is a static continuous ability with no
    // trigger word and no activation cost; not expressible in this shape.
    // GAP: "Enchanted creature gets +2/+1 and can't block." is the Aura-mode
    // static granted to the enchanted creature; not expressible in this shape.
    reg.register(CardDefinition::new(name, chars))
}
