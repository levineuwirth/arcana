//! Scrapheap Scrounger — `{2}` 3/2 Artifact Creature — Construct.
//! "This creature can't block." (static)
//! "{1}{B}, Exile another creature card from your graveyard: Return this card
//!  from your graveyard to the battlefield."
//!
//! The "can't block" static has no expressible primitive (GAP). The recursion
//! ability's cost includes "exile another creature card from your graveyard",
//! which is not a representable ActivationCost field (only discard_other /
//! sacrifice_other / exile_self exist — no exile-other-from-graveyard cost), so
//! the activated ability is GAP'd to avoid emitting a free recursion engine.

use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scrapheap Scrounger");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    // GAP: "This creature can't block." — a can't-block static with no
    // expressible primitive.
    // GAP: "{1}{B}, Exile another creature card from your graveyard: Return this
    // card from your graveyard to the battlefield." — the exile-other-from-
    // graveyard activation cost is not a representable ActivationCost field.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
