//! Floodbringer — `{1}{U}` 1/2 Moonfolk Wizard.
//!
//! * Flying.
//! * `{2}, Return a land you control to its owner's hand: Tap target
//!   land.` — the "return a land you control to its owner's hand"
//!   additional cost has no demonstrated `ActivationCost` field (only
//!   sacrifice/tap/discard of other permanents are modeled). Expressing
//!   the tap without the return cost would be a materially wrong card,
//!   so the whole activated ability is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Floodbringer");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "{2}, Return a land you control to its owner's hand: Tap
    // target land." — no "return a permanent to hand" activation-cost
    // field is demonstrated; the whole ability is omitted rather than
    // dropping the cost.
    reg.register(CardDefinition::new(name, chars))
}
