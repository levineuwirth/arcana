//! Walking Sarcophagus — `{2}` 2/1 colorless Artifact Creature — Zombie Cat.
//!
//! GAP: "Start your engines!" / "Max speed" — the speed mechanic
//! (keywords + max-speed-gated static "+1/+2") is not in the usable
//! KeywordAbility surface and the speed-gated static buff is not
//! expressible with the demonstrated Effect surface.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Walking Sarcophagus");
    let zombie = reg.interner_mut().intern("Zombie");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
