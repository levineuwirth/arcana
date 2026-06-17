//! Satyr Nyx-Smith — `{2}{R}` 2/1 Satyr Shaman with Haste.
//!
//! Oracle:
//! * Haste.
//! * Inspired — Whenever this creature becomes untapped, you may pay {2}{R}.
//!   If you do, create a 3/1 red Elemental enchantment creature token with
//!   haste.
//!
//! GAP: there is no "becomes untapped" TriggerCondition variant, so the
//! Inspired trigger cannot be wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Satyr Nyx-Smith");
    let satyr = reg.interner_mut().intern("Satyr");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
