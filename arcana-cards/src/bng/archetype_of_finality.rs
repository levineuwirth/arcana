//! Archetype of Finality — `{4}{B}{B}` 2/3 Enchantment Creature —
//! Gorgon.
//!
//! Oracle:
//! * "Creatures you control have deathtouch." — a static anthem-style
//!   keyword grant; no demonstrated triggered/activated primitive
//!   expresses an always-on board-wide keyword grant. GAP.
//! * "Creatures your opponents control lose deathtouch and can't have
//!   or gain deathtouch." — a static keyword-removal/lock; GAP.
//!
//! Both lines are pure continuous statics with no trigger word or cost,
//! so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP static: "Creatures you control have deathtouch."
// GAP static: "Creatures your opponents control lose deathtouch and
//             can't have or gain deathtouch."

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archetype of Finality");
    let gorgon = reg.interner_mut().intern("Gorgon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gorgon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
