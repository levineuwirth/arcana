//! Noble Quarry — `{2}{G}` 1/1 Enchantment Creature — Unicorn.
//! Bestow {5}{G}.
//! All creatures able to block this creature or enchanted creature do so.
//! Enchanted creature gets +1/+1.
//!
//! Bones only. Bestow is GAP'd (not in the usable keyword surface; the
//! aura/creature dual-cast mechanic is not expressible here). The lure-style
//! "all creatures able to block ... do so" and the "enchanted creature gets
//! +1/+1" lines are pure STATIC abilities (no trigger word / no cost), which
//! the multi-ability creature shape composes only as triggered/activated
//! abilities — both GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Noble Quarry");
    let unicorn = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Bestow {5}{G} — not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: "All creatures able to block this creature or enchanted creature
    // do so." — lure static, not a triggered/activated ability.
    // GAP: "Enchanted creature gets +1/+1." — aura static.
    reg.register(CardDefinition::new(name, chars))
}
