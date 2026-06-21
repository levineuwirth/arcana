//! Drogskol Reinforcements — `{3}{W}` 2/2 Spirit Soldier.
//! Melee.
//! Other Spirits you control have melee.
//! Prevent all noncombat damage that would be dealt to Spirits you control.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drogskol Reinforcements");
    let spirit = reg.interner_mut().intern("Spirit");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Melee (Scryfall keyword) is not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Other Spirits you control have melee" is a static keyword grant to
    // other permanents — no expressible variant in this card shape.
    // GAP: "Prevent all noncombat damage that would be dealt to Spirits you
    // control" is a continuous static prevention with no expressible variant
    // (prevention effects require a trigger/ability slot to apply).
    reg.register(CardDefinition::new(name, chars))
}
