//! Johann, Apprentice Sorcerer — `{2}{U}{R}` 2/5 Legendary Creature —
//! Human Wizard Sorcerer.
//!
//! Oracle:
//! * You may look at the top card of your library any time. (Static
//!   information-access ability — GAP'd.)
//! * Once each turn, you may cast an instant or sorcery spell from the top of
//!   your library. (Static casting permission — GAP'd.)
//!
//! Both lines are static play-permission / information abilities with no
//! triggered or activated ability to decompose, so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Johann, Apprentice Sorcerer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "You may look at the top card of your library any time." — static
    // information-access ability; no demonstrated hook.
    // GAP: "Once each turn, you may cast an instant or sorcery spell from the
    // top of your library." — static play-from-library permission; no
    // alternate-cast-zone mechanic in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
