//! Lavabrink Venturer — `{2}{W}` 3/3 Human Soldier.
//! "As this creature enters, choose odd or even. This creature has
//!  protection from each mana value of the chosen quality."
//!
//! GAP: the "choose odd or even" replacement-style enters choice and the
//! resulting "protection from each mana value of the chosen quality"
//! static are not expressible with the demonstrated effect/keyword surface
//! (no protection keyword, no odd/even choice primitive). Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lavabrink Venturer");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
