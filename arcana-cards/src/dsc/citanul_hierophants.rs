//! Citanul Hierophants — `{3}{G}` 3/2 green Human Druid. "Creatures you
//! control have '{T}: Add {G}.'"
//!
//! GAP: "grant '{T}: Add {G}' to all creatures you control" is a static
//! ability — no global activated-ability grant variant.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Citanul Hierophants");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "all creatures you control have '{T}: Add {G}'" is a static layer-6
    // ability grant — not expressible as an activated ability on this card.
    reg.register(CardDefinition::new(name, chars))
}
