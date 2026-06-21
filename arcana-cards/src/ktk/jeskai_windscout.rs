//! Jeskai Windscout — `{2}{U}` 2/1 Bird Scout.
//!
//! Rules text:
//! * Flying
//! * Prowess (Whenever you cast a noncreature spell, this creature gets +1/+1
//!   until end of turn.)
//!
//! Flying is faithful. Prowess is not a usable KeywordAbility variant in this
//! card class (and there is no dedicated trigger condition for it in the
//! demonstrated API), so it is GAP'd — only Flying is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jeskai Windscout");
    let bird = reg.interner_mut().intern("Bird");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Prowess is not an available KeywordAbility variant for this class.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
