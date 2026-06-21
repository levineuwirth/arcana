//! Contortionist Troupe — `{X}{G}` 0/0 Creature — Human.
//!
//! Oracle:
//! * "This creature enters with X +1/+1 counters on it." →
//!   `EntersWithSpec::CountersFromX { kind: PlusOnePlusOne }`.
//! * "Coven — At the beginning of your end step, if you control three
//!   or more creatures with different powers, put a +1/+1 counter on
//!   target creature you control." → the Coven intervening-if ("three
//!   or more creatures with different powers") has no `conditions::`
//!   predicate, so the whole gated trigger is a GAP rather than firing
//!   unconditionally.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

// GAP keyword/trigger: "Coven" and its end-step trigger — the
//   "three or more creatures with different powers" intervening-if has
//   no available condition predicate.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Contortionist Troupe");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::CountersFromX {
                kind: CounterKind::PlusOnePlusOne,
            }),
    )
}
