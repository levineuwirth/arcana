//! Polygoyf — `{2}{G}` */1+* Lhurgoyf with Trample.
//!
//! Oracle:
//! * Trample — keyword. (Scryfall also lists `Myriad`; it is not a
//!   supported `KeywordAbility` and its attack-trigger token-copy fan-out
//!   is not expressible, so it is GAP'd.)
//! * Polygoyf's power is equal to the number of card types among cards in
//!   all graveyards and its toughness is equal to that number plus 1. — a
//!   characteristic-defining ability. No `script::` helper counts card
//!   types across all graveyards and no primitive sets base P/T from such a
//!   count, so P/T are left as `*` / `*+1` (the CDA itself is GAP'd).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Polygoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: CDA — P/T = number of card types among all graveyards
        // (+1 for toughness). Left as `*` / `*+1`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
