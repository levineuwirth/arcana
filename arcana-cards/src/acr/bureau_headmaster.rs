//! Bureau Headmaster — `{R}{W}` 2/2 Human Assassin.
//! "Equipment spells you cast cost {1} less to cast."
//! "Equip abilities you activate cost {1} less to activate."
//!
//! Both lines are static cost-reduction abilities with no triggered/activated
//! decomposition and no Effect variant to express them, so both are GAP'd —
//! only bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "Equipment spells you cast cost {1} less" — static cost reduction.
// GAP: "Equip abilities you activate cost {1} less" — static cost reduction.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bureau Headmaster");
    let human = reg.interner_mut().intern("Human");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
