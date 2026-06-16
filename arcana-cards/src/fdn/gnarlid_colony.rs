//! Gnarlid Colony — `{1}{G}` 2/2 Beast.
//! Kicker {2}{G}; "If this creature was kicked, it enters with two +1/+1
//! counters on it."; static "Each creature you control with a +1/+1 counter
//! on it has trample."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gnarlid Colony");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker is an alternative-cost casting mode, not a KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: "If this creature was kicked, it enters with two +1/+1 counters" —
    // kicked-state is not exposed to a SelfEntersBattlefield effect; no
    // was-kicked condition available.
    // GAP: static "Each creature you control with a +1/+1 counter on it has
    // trample" — a continuous keyword-granting static, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
