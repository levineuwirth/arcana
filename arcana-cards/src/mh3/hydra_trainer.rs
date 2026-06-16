//! Hydra Trainer — `{1}{G}` 1/1 Human Warrior.
//! You may exert this creature as it attacks. When you do, target
//! creature gets +X/+X until end of turn, where X is the number of
//! counters on permanents you control.
//! {2}{G}: Adapt 2.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hydra Trainer");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Adapt / Exert are not in the usable keyword surface for
        // this card class.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "You may exert this creature as it attacks. When you do, ..."
    // — no exert-on-attack TriggerCondition / cost is expressible.
    // GAP: "{2}{G}: Adapt 2." — no Effect::Adapt (or equivalent) primitive
    // is available in the effect catalog; emitting it would require an
    // invented variant.
    reg.register(CardDefinition::new(name, chars))
}
