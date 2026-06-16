//! Nim Devourer — `{3}{B}{B}` 4/1 Zombie.
//! "This creature gets +1/+0 for each artifact you control.
//!  {B}{B}: Return this card from your graveyard to the battlefield, then
//!  sacrifice a creature. Activate only during your upkeep."
//!
//! Both non-bones lines are GAP'd:
//!  * The "+1/+0 for each artifact you control" line is a pure static
//!    continuous ability (no trigger word, no cost) — not expressible.
//!  * The graveyard activation "Return THIS card from your graveyard to the
//!    battlefield" has no self-return-from-graveyard primitive (the catalog's
//!    ReturnFromGraveyardToBattlefield needs a chosen target id; Reanimate is
//!    filter-based and would return any creature, not necessarily this card),
//!    and the "Activate only during your upkeep" timing restriction is not
//!    expressible via the documented ActivationCost surface — so the whole
//!    activated ability is omitted rather than emit an unfaithful one.
//!
//! Only the bones (name, cost, colors, type, P/T) are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nim Devourer");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static — "gets +1/+0 for each artifact you control" (pure static).
    // GAP: activated — "{B}{B}: Return this card from your graveyard to the
    // battlefield, then sacrifice a creature. Activate only during your
    // upkeep." (no self-graveyard-return primitive + no upkeep-timing gate).
    reg.register(CardDefinition::new(name, chars))
}
