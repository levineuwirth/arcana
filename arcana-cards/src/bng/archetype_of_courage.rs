//! Archetype of Courage — `{1}{W}{W}` 2/2 Enchantment Creature — Human Soldier.
//! Creatures you control have first strike.
//! Creatures your opponents control lose first strike and can't have or gain
//! first strike.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archetype of Courage");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP static: "Creatures you control have first strike" — a continuous
    // anthem-style keyword grant, not a triggered/activated ability.
    // GAP static: "Creatures your opponents control lose first strike and can't
    // have or gain first strike" — continuous removal/lock, unexpressible here.
    reg.register(CardDefinition::new(name, chars))
}
