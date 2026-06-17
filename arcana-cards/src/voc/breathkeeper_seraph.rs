//! Breathkeeper Seraph — `{4}{W}{W}` 4/4 Angel with Flying and Soulbond.
//! As long as it is paired, each paired creature has "When this creature
//! dies, you may return it to the battlefield at the beginning of your
//! next upkeep."
//!
//! Soulbond is not in the usable keyword surface, and the soulbond-gated
//! static grant is not expressible as a triggered/activated ability —
//! only Flying is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Breathkeeper Seraph");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    // GAP: Soulbond is not in the usable keyword surface.
    // GAP: the soulbond-gated static grant ("each paired creature has ...")
    //      is a continuous ability, not a triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
