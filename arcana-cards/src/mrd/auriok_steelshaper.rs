//! Auriok Steelshaper — `{1}{W}` 1/1 Human Soldier.
//! "Equip costs you pay cost {1} less. As long as this creature is
//! equipped, each creature you control that's a Soldier or a Knight gets
//! +1/+1."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Auriok Steelshaper");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "Equip costs you pay cost {1} less." — a static equip-cost
        // reduction has no primitive on this card class.
        // GAP: "As long as this creature is equipped, each Soldier or Knight
        // you control gets +1/+1." — an equipped-conditional filtered anthem
        // static is not expressible (no trigger or cost; no equipped-state
        // gated continuous effect).
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
