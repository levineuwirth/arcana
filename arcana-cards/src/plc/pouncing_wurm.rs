//! Pouncing Wurm — `{3}{G}` 3/3 green Wurm.
//!
//! Oracle:
//! * Kicker {2}{G}
//! * If this creature was kicked, it enters with three +1/+1 counters on it
//!   and with haste.
//!
//! Kicker is not a KeywordAbility variant and the kicker cast mechanic is
//! not modeled, so both the keyword and the kicked-conditional ETB
//! (counters + haste) are GAP'd. What remains is the vanilla 3/3 Wurm bones.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pouncing Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Kicker {2}{G} is not a KeywordAbility variant; the kicker cast
        // mechanic is unmodeled.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "If this creature was kicked, it enters with three +1/+1 counters
    // on it and with haste." — depends on the unmodeled kicker mechanic; no
    // demonstrated primitive tracks whether a spell was kicked.
    reg.register(CardDefinition::new(name, chars))
}
