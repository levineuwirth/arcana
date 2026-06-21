//! Cliffrunner Behemoth — `{3}{G}` 5/3 Rhino Beast.
//! "This creature has haste as long as you control a red permanent.
//!  This creature has lifelink as long as you control a white permanent."
//!
//! Both lines are CONDITIONAL STATIC keyword grants (CR 604) — keyword
//! abilities that exist only while a board condition holds. The
//! demonstrated API exposes only base-characteristic `keywords` and
//! one-shot `Effect::GrantKeyword`; there is no documented primitive
//! for a condition-gated static keyword. Both are GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cliffrunner Behemoth");
    let rhino = reg.interner_mut().intern("Rhino");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "has haste as long as you control a red permanent" — condition-gated static keyword grant not expressible with the demonstrated API.
    // GAP: static "has lifelink as long as you control a white permanent" — condition-gated static keyword grant not expressible with the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
