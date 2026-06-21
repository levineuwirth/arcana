//! Akawalli, the Seething Tower — `{1}{B}{G}` 3/3 Legendary Fungus.
//! Descend 4 — static: while 4+ permanent cards in your graveyard, gets
//! +2/+2 and has trample.
//! Descend 8 — static: while 8+ permanent cards in your graveyard, gets an
//! additional +2/+2 and can't be blocked by more than one creature.
//!
//! Both lines are conditional STATIC continuous abilities (no trigger word,
//! no cost) — not expressible as triggered/activated abilities. Descend is
//! not a supported keyword. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akawalli, the Seething Tower");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Descend is not a supported keyword.
        ..Default::default()
    };
    // GAP: "Descend 4 — gets +2/+2 and has trample" — conditional static.
    // GAP: "Descend 8 — gets additional +2/+2 and can't be blocked by more
    // than one creature" — conditional static.
    reg.register(CardDefinition::new(name, chars))
}
