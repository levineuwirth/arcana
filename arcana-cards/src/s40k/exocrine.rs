//! Exocrine — `{X}{2}{R}` 2/2 Tyranid.
//! Ravenous (gapped — not an available keyword; the "enters with X +1/+1
//! counters, draw if X >= 5" rider is not expressible). Bio-plasmic
//! Barrage — when it enters, it deals X damage to each player and each
//! other creature (gapped — the value X paid for the cost is not readable
//! from an ETB trigger's resolution context).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exocrine");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Ravenous — not an available KeywordAbility; "enters with X
        // +1/+1 counters; if X >= 5, draw a card" is not expressible.
        ..Default::default()
    };

    // GAP: "Bio-plasmic Barrage — When this creature enters, it deals X damage
    // to each player and each other creature" — the X paid for the {X} cost is
    // not readable in an ETB triggered ability's resolution context.
    reg.register(CardDefinition::new(name, chars))
}
