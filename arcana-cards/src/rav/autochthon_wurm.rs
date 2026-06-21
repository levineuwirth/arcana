//! Autochthon Wurm — `{10}{G}{G}{G}{W}{W}` 9/14 Wurm. Trample, Convoke.
//! Convoke is a cast-time cost-reduction mechanic not in the supported
//! keyword surface — GAP'd. Trample is a base keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Autochthon Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}{G}{G}{G}{W}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(14)),
        // GAP: Convoke — cast-time cost reduction, not in the supported keyword
        // surface.
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
