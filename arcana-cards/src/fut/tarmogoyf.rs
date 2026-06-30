//! Tarmogoyf — `{1}{G}` Creature — Lhurgoyf. Printed P/T is `*/1+*`, where `*`
//! is the number of card types among all graveyards (a characteristic-defining
//! ability, CR 604.3 / Layer 7a).
//!
//! GAP: the dynamic CDA P/T is a deferred engine task — `GameState::
//! computed_power` explicitly returns `None` for `PtValue::Star`/`StarPlus`
//! (no Layer-7a CDA resolution, and the layer-aware power path has no registry
//! to source a per-card star value). Modeled here as a fixed 4/5: a
//! representative competitive body for an efficient two-mana beater, which is
//! what a material-referee gauntlet actually evaluates. Revisit when a CDA hook
//! (object-carried star-value fn + computed_power/toughness resolution) lands.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tarmogoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
