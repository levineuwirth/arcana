//! Phyrexian Scuta — `{3}{B}` 3/3 Phyrexian Zombie.
//! "Kicker—Pay 3 life."
//! "If this creature was kicked, it enters with two +1/+1 counters on it."
//! GAP: Kicker is not in the usable keyword surface, and the engine exposes no
//! cast-time "was kicked" flag / non-mana additional cost. Both the kicker
//! payment and the kicked-ETB +1/+1 counter rider are therefore unexpressible —
//! emitted as a vanilla 3/3 with the gaps noted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Scuta");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
