//! Nethergoyf — `{B}` */1+* Lhurgoyf.
//! "Nethergoyf's power is equal to the number of card types among cards
//! in your graveyard and its toughness is equal to that number plus 1."
//! Escape—{2}{B}, exile any number of other cards from your graveyard
//! with four or more card types among them.
//!
//! P/T are captured as `*` / `*+1` (PtValue::Star / StarPlus(1)); the
//! characteristic-defining ability describing the actual graveyard-card-
//! type count is a documented partial. Escape is not in the usable
//! keyword surface — GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nethergoyf");
    let lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // CDA partial: power `*` = # card types in your graveyard,
        // toughness `*+1`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        // GAP: Escape—{2}{B}, exile any number of other cards … — not in
        // the usable keyword surface.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
