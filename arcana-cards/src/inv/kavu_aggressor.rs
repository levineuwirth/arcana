//! Kavu Aggressor — `{2}{R}` 3/2 Kavu.
//! "Kicker {4}.
//!  This creature can't block.
//!  If this creature was kicked, it enters with a +1/+1 counter on it."

use arcana_core::effects::{KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kavu Aggressor");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Kicker(
            ManaCost::parse("{4}").expect("valid kicker cost"),
        )],
        ..Default::default()
    };

    // GAP: static "This creature can't block" — no static can't-block
    // expression in this shape (Effect::ForbidBlocking is a targeted,
    // duration-bound effect, not a permanent self-static).
    // GAP: "If this creature was kicked, it enters with a +1/+1 counter"
    // — no per-cast "was kicked" predicate exposed to a SelfEnters
    // trigger / intervening_if; can't gate the ETB counter on kicked.
    reg.register(CardDefinition::new(name, chars))
}
