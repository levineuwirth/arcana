//! Fledgling Dragon — `{2}{R}{R}` 2/2 Creature — Dragon. Red.
//! Flying.
//! "Threshold — As long as there are seven or more cards in your
//!  graveyard, this creature gets +3/+3 and has '{R}: This creature gets
//!  +1/+0 until end of turn.'"
//!
//! The Threshold clause is a graveyard-gated STATIC: a conditional +3/+3
//! buff plus a conditionally-granted activated ability. Neither half is
//! expressible — there is no conditional-static primitive, and the
//! granted "{R}: +1/+0" exists only while threshold is active (it can't
//! be emitted as an unconditional activated ability without overstating
//! the card). Both are GAP'd. (Threshold is an ability word, not a
//! keyword line.) Bones + Flying only.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fledgling Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: Threshold-gated static (+3/+3 and a granted "{R}: +1/+0"
    // activated ability while seven+ cards are in your graveyard) — no
    // conditional-static primitive in the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
