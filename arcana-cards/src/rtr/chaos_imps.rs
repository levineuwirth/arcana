//! Chaos Imps — `{4}{R}{R}` 6/5 Imp.
//! Flying.
//! Unleash.
//! "This creature has trample as long as it has a +1/+1 counter on it."
//!
//! Flying and Unleash are base keywords. The conditional "has trample as
//! long as it has a +1/+1 counter" static is not expressible (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chaos Imps");
    let imp = reg.interner_mut().intern("Imp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(imp);

    // GAP: "This creature has trample as long as it has a +1/+1 counter on
    // it." — a counter-conditional continuous keyword grant is not
    // expressible with the available primitives.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Unleash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
