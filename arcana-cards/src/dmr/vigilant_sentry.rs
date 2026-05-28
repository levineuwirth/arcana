//! Vigilant Sentry — `{1}{W}{W}` 2/2 white Human Nomad.
//! Threshold — "As long as there are seven or more cards in your graveyard,
//! this creature gets +1/+1 and has '{T}: Target attacking or blocking
//! creature gets +3/+3 until end of turn.'"
//! GAP: Threshold static ability (continuous effect based on graveyard size)
//! is not in the triggered/activated framework; emitting as bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vigilant Sentry");
    let human = reg.interner_mut().intern("Human");
    let nomad = reg.interner_mut().intern("Nomad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(nomad);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: Threshold static ability (continuous effect based on graveyard
    // size) is not expressible.
    reg.register(CardDefinition::new(name, chars))
}
