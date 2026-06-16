//! A-Gnarlid Colony — `{1}{G}` 2/2 Beast.
//! Kicker {2}{G}; "If Gnarlid Colony was kicked, it enters with four +1/+1
//! counters on it."; "Each creature you control with a +1/+1 counter on it has
//! trample."
//!
//! Kicker is not in the usable KeywordAbility surface (GAP). The kicked-ETB
//! counters depend on the (unexpressible) kicker payment (GAP). The trample-
//! granting line is a pure static continuous ability with no triggered/activated
//! shape (GAP). No expressible abilities — bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Gnarlid Colony");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // GAP: Kicker {2}{G} is not in the usable KeywordAbility surface.
    // GAP: "if kicked, enters with four +1/+1 counters" depends on kicker payment.
    // GAP: static — "each creature you control with a +1/+1 counter has trample".

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
