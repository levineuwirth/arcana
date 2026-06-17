//! Fin-Clade Fugitives — `{5}{G}` 7/4 green Elf Salamander Rogue.
//!
//! Both non-bones lines are unexpressible:
//! * "This creature can't be blocked by creatures with power 2 or less." — a
//!   power-filtered evasion static (no `CantBeBlockedBy{filter}` primitive in
//!   the supported surface) — GAP.
//! * Encore {4}{G} — Encore is not a supported keyword, and the per-opponent
//!   token-copy/attacks/haste/sacrifice payoff has no expressible primitive —
//!   GAP.
//! Emitting bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "This creature can't be blocked by creatures with power 2 or less." —
// power-filtered evasion static, not expressible.
// GAP: "Encore {4}{G}" — Encore keyword unsupported; the graveyard-activated
// per-opponent token-copy payoff is not expressible; keywords vec is empty.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fin-Clade Fugitives");
    let elf = reg.interner_mut().intern("Elf");
    let salamander = reg.interner_mut().intern("Salamander");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(salamander);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
