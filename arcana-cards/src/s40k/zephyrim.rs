//! Zephyrim — `{3}{W}` 3/3 Human Warrior.
//!
//! "Squad {2} (As an additional cost to cast this spell, you may pay {2}
//!  any number of times. When this creature enters, create that many
//!  tokens that are copies of it.)
//!  Flying, vigilance.
//!  Miracle {1}{W} (You may cast this card for its miracle cost when you
//!  draw it if it's the first card you drew this turn.)"
//!
//! Flying and vigilance are base keywords. Squad and Miracle are
//! alternative/additional cast modifiers with no supported keyword variant
//! or cast hook; the Squad ETB depends on the additional-cost payment, so
//! both are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zephyrim");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: Squad {2} — additional cast cost (pay {2} any number of times)
    // plus an ETB that creates that-many copies; not expressible (no cast
    // modifier hook, ETB count depends on cost paid).
    // GAP: Miracle {1}{W} — alternative cast cost on draw; no cast hook.
    reg.register(CardDefinition::new(name, chars))
}
