//! Guardian of Vitu-Ghazi — `{6}{G}{W}` 4/7 Elemental with Vigilance.
//!
//! * "Convoke" — GAP: Convoke is not in the KeywordAbility surface and the
//!   tap-creatures-to-pay cast cost is not expressible.
//! * Vigilance — keyword line.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guardian of Vitu-Ghazi");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
