//! Void Winnower — `{9}` 11/9 colorless Eldrazi.
//! "Your opponents can't cast spells with even mana values."
//! "Your opponents can't block with creatures with even mana values."
//!
//! Both lines are pure static continuous restrictions keyed on an
//! even-mana-value predicate. There is no even/odd mana-value filter on
//! ObjectFilter and no can't-cast / can't-block static primitive in the
//! demonstrated API, so both abilities are GAP'd — only the bones are
//! emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Void Winnower");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    // GAP: "Your opponents can't cast spells with even mana values."
    // GAP: "Your opponents can't block with creatures with even mana values."
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(11)),
        toughness: Some(PtValue::Fixed(9)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
