//! Little Girl — `{HW}` (half-white) 0.5/0.5 vanilla Human Child.
//! An Unhinged joke card with fractional power/toughness and a
//! half-mana cost. No rules text beyond its stats.
//!
//! GAP: the printed mana cost is `{HW}` (half-white) and the
//! power/toughness are `.5/.5` (fractional). Neither half-mana symbols
//! nor fractional `PtValue` are expressible with the demonstrated API
//! (`ManaCost::parse` has no half-symbol form; `PtValue::Fixed` is an
//! integer). Approximated as `{W}` 0/0 — flag for human routing.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Little Girl");
    let human = reg.interner_mut().intern("Human");
    let child = reg.interner_mut().intern("Child");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(child);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
