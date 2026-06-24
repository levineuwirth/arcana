//! Skyshroud War Beast — `{1}{G}` */* Beast with Trample.
//!
//! * Trample.
//! * As this creature enters, choose an opponent.
//! * Skyshroud War Beast's power and toughness are each equal to the number
//!   of nonbasic lands the chosen player controls.
//!
//! The "choose an opponent" replacement plus the chosen-player-relative
//! characteristic-defining ability (P/T = that player's nonbasic land count)
//! is GAP'd: the self-CDA constructors resolve `*` from a count the SOURCE'S
//! CONTROLLER sees (a fixed `you`/everyone filter or a scalar over the
//! source's controller); they cannot count over an as-enters-CHOSEN, then
//! REMEMBERED opponent. No chosen-player-memory channel exists for the CDA to
//! read, so this is a genuine player-choice P/T GAP. P/T stay `Star` to
//! record the `*/*` bones; Trample is a base keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyshroud War Beast");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: CDA — P/T each equal to the chosen opponent's nonbasic land
        //       count; no CDA primitive / chosen-player memory in the API.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
