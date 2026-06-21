//! Markov Baron — `{2}{B}` 2/2 Vampire Noble with Lifelink.
//!
//! Oracle:
//! * Convoke.
//! * Lifelink.
//! * Other Vampires you control get +1/+1.
//! * Madness {2}{B}.
//!
//! Lifelink is a base characteristic. Convoke and Madness are not usable
//! KeywordAbility variants for this card class (cost-modification mechanics);
//! "Other Vampires you control get +1/+1" is a static anthem continuous effect
//! over other objects with no primitive. All three are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Markov Baron");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: Convoke — cost-reduction keyword not in the usable surface.
    // GAP: "Other Vampires you control get +1/+1" — static anthem over other
    // objects; no primitive.
    // GAP: Madness {2}{B} — alternative-cast keyword not in the usable surface.
    reg.register(CardDefinition::new(name, chars))
}
