//! Magma Hellion — `{6}{R}` 5/4 Hellion with Trample and Haste.
//! Assist (another player can pay up to {6} of this spell's cost). (GAP — not modeled)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magma Hellion");
    let hellion = reg.interner_mut().intern("Hellion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Assist (cost-sharing keyword) has no KeywordAbility variant; omitted.
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
