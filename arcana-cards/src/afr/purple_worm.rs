//! Purple Worm — `{5}{G}{G}` 8/7 Worm with Ward {2}. "This spell costs {2}
//! less to cast if a creature died this turn." (cast-cost modifier — GAP'd.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "This spell costs {2} less to cast if a creature died this turn." — a
// cast-time cost reduction, not expressible in this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Purple Worm");
    let worm = reg.interner_mut().intern("Worm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(worm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Ward(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
