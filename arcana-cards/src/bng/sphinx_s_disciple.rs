//! Sphinx's Disciple — `{3}{U}{U}` 2/2 Human Wizard.
//!
//! * Flying (keyword).
//! * Inspired — "Whenever this creature becomes untapped, draw a
//!   card." GAP: there is no `TriggerCondition` for a permanent
//!   becoming UNTAPPED in the usable surface (only `SelfBecomesTapped`
//!   exists). The Inspired trigger is emitted with the closest-fit
//!   firing impossible, so the ability is dropped and only the keyword
//!   is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphinx's Disciple");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: Inspired — "Whenever this creature becomes untapped, draw a card."
        // No becomes-untapped TriggerCondition variant in the usable surface.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
