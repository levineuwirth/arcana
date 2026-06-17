//! Greater Gargadon — `{9}{R}` 9/7 Beast.
//! Suspend 10—{R} — GAP: Suspend is not in the usable keyword surface.
//! "Sacrifice an artifact, creature, or land: Remove a time counter from
//!  this card. Activate only if this card is suspended." — GAP: this
//!  ability is activated from exile while the card is suspended; neither
//!  the suspended-exile activation zone nor the "only if suspended"
//!  precondition is expressible, so the whole ability is omitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Greater Gargadon");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
