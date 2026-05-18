//! Wolverine Pack — `{2}{G}{G}` 2/4 Wolverine with Rampage 2.
//! Rampage 2 (whenever this creature becomes blocked, it gets +2/+2 until
//! end of turn for each creature blocking it beyond the first). Rampage is
//! not in the demonstrated KeywordAbility set; the verify pipeline will
//! flag the gap.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolverine Pack");
    let wolverine = reg.interner_mut().intern("Wolverine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolverine);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
