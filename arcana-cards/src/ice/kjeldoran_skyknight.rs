//! Kjeldoran Skyknight — `{2}{W}` 1/1 Human Knight with Flying,
//! First Strike, and Banding.
//!
//! # Rules references
//!
//! * CR 702.9  — Flying. Can only be blocked by creatures with Flying
//!   or Reach.
//! * CR 702.7  — First Strike. Deals combat damage before creatures
//!   without first strike.
//! * CR 702.22 — Banding. Allows creatures to attack or block as a
//!   band; the controlling player assigns the banded creature's combat
//!   damage.
//!
//! NOTE: `Banding` is not present in the demonstrated `KeywordAbility`
//! variant list. Flying and FirstStrike are encoded; Banding is omitted
//! from the keywords vec. The verify pipeline will flag the gap.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kjeldoran Skyknight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
