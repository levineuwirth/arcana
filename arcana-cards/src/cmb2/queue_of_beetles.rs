//! Queue of Beetles — `{3}{R}` 3/3 Insect with Haste.
//!
//! * Haste (keyword).
//! * "The stack is now first in, first out instead of last in, first
//!   out." GAP: a static rule-altering ability that changes the order
//!   in which the stack resolves; it has no trigger word, no cost, and
//!   no corresponding `Effect` in the usable surface, so it is not
//!   expressible. Emitted with bones and the Haste keyword only.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Queue of Beetles");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        // GAP: "The stack is now first in, first out" — static rule-altering ability, no Effect/trigger form.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
