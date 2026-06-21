//! Tekuthal, Inquiry Dominus — `{2}{U}{U}` 3/5 Legendary Phyrexian Horror.
//!
//! Oracle:
//! * Flying.
//! * If you would proliferate, proliferate twice instead.
//! * `{1}{U/P}{U/P}, Remove three counters from among other artifacts,
//!   creatures, and planeswalkers you control: Put an indestructible counter
//!   on Tekuthal.`
//!
//! Only Flying is expressible. The proliferate-doubling replacement has no
//! primitive and is GAP'd. The activated ability's cost — "Remove three
//! counters from among OTHER permanents you control" — has no cost field
//! (`remove_self_counter` only removes the source's own counters), so the
//! whole activated ability is GAP'd. (Proliferate is not a `KeywordAbility`
//! variant, so it is omitted from the keyword line.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tekuthal, Inquiry Dominus");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    // GAP: static — "If you would proliferate, proliferate twice instead"
    // (proliferate-doubling replacement; no primitive).
    // GAP: activated — "{1}{U/P}{U/P}, Remove three counters from among other
    // permanents you control: Put an indestructible counter on Tekuthal" — no
    // remove-counters-from-other-permanents cost field.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
