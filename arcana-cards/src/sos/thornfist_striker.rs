//! Thornfist Striker — `{2}{G}` 3/3 Creature — Elf Druid.
//!
//! * Ward {1} — parametrized evergreen keyword.
//! * "Infusion — Creatures you control get +1/+0 and have trample as
//!   long as you gained life this turn." — a conditional static anthem
//!   (and the Infusion keyword itself is not in the usable surface);
//!   GAP'd (no triggered/activated hook for a conditional static).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thornfist Striker");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{1}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: "Infusion — Creatures you control get +1/+0 and have trample
    // as long as you gained life this turn." — conditional static anthem
    // with no triggered/activated hook; Infusion is not in the usable
    // keyword surface.

    reg.register(CardDefinition::new(name, chars))
}
