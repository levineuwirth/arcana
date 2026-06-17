//! Knight of Lost Causes — `{W}{W}` 2/2 Human Knight with Vigilance.
//! "As long as you are way behind, Knight of Lost Causes gets +3/+3 and has
//! indestructible."
//!
//! Vigilance is a base keyword. The "way behind" conditional static buff (+3/+3 and
//! indestructible) is a continuous conditional ability with no trigger/cost and no
//! "way behind" predicate — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of Lost Causes");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    // GAP: "As long as you are way behind, ~ gets +3/+3 and has indestructible." —
    // conditional continuous static with a bespoke "way behind" predicate.

    reg.register(CardDefinition::new(name, chars))
}
