//! Yarok, the Desecrated — `{2}{B}{G}{U}` 3/5 Legendary Elemental Horror with
//! Deathtouch and Lifelink.
//! "If a permanent entering causes a triggered ability of a permanent you
//!  control to trigger, that ability triggers an additional time."
//!
//! The only non-keyword line is a doubling replacement static (Panharmonicon
//! class); it is not a triggered/activated ability, so this card carries bones
//! and keywords only.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yarok, the Desecrated");
    let elemental = reg.interner_mut().intern("Elemental");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Lifelink],
        ..Default::default()
    };

    // GAP: static — "If a permanent entering causes a triggered ability of a
    //       permanent you control to trigger, that ability triggers an
    //       additional time." (ETB-trigger-doubling replacement static).
    reg.register(CardDefinition::new(name, chars))
}
