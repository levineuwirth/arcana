//! Dragon's Disciple — `{1}{W}` 1/3 Human Monk.
//!
//! Rules text (both abilities are statics with no expressible
//! triggered/activated primitive — emitted as bones only):
//! * "As this creature enters, you may reveal a Dragon card from your
//!   hand. If you do or if you control a Dragon, this creature enters
//!   with a +1/+1 counter on it." — a conditional, choice-driven
//!   enters-with-counters replacement. GAP'd: no primitive expresses
//!   a may-reveal-gated ETB counter.
//! * "Dragons you control have ward {1}." — a static ability that
//!   grants a keyword to other permanents you control. GAP'd: no
//!   keyword-granting static primitive available.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragon's Disciple");
    let human = reg.interner_mut().intern("Human");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: conditional may-reveal-gated enters-with-counter ETB.
    // GAP: static "Dragons you control have ward {1}." keyword grant.
    reg.register(CardDefinition::new(name, chars))
}
