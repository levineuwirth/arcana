//! Archetype of Imagination — `{4}{U}{U}` 3/2 blue Enchantment Creature —
//! Human Wizard.
//!
//! Oracle:
//! * Creatures you control have flying.
//! * Creatures your opponents control lose flying and can't have or gain
//!   flying.
//!
//! Both lines are pure static continuous abilities (no trigger, no cost).
//! The MultiAbilityCreature surface exposes only keyword / triggered /
//! activated abilities, so both statics are GAP'd below — there is no
//! continuous-anthem primitive in the demonstrated API.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Archetype of Imagination");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    // GAP: static "Creatures you control have flying." — no continuous
    // keyword-anthem primitive in the MultiAbilityCreature surface.
    // GAP: static "Creatures your opponents control lose flying and can't
    // have or gain flying." — no continuous keyword-removal/lock primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
