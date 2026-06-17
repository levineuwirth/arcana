//! Tyrant Guard — `{X}{2}{G}` 3/3 Tyranid.
//! Ravenous and Shieldwall are not supported keywords — keyword line empty.
//! "Ravenous (enters with X +1/+1 counters; if X ≥ 5, draw a card when it enters)" —
//! enters-with-X-counters / cast-X coupling not expressible — GAP.
//! "Shieldwall — Sacrifice this creature: Creatures you control with counters on them
//! gain hexproof and indestructible until end of turn." — the "with counters on them"
//! board filter has no ObjectFilter predicate in the listed API — GAP.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyrant Guard");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
