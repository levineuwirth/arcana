//! Soratami Savant — `{2}{U}{U}` 2/2 Moonfolk Wizard with Flying.
//! "{3}, Return a land you control to its owner's hand: Counter target spell
//! unless its controller pays {3}."
//!
//! GAP: the activated ability is omitted — its cost includes "return a land you
//! control to its owner's hand", which is not an `ActivationCost` field, and the
//! "counter target spell unless its controller pays {3}" effect has no
//! corresponding `Effect` variant (no counter primitive).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soratami Savant");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
