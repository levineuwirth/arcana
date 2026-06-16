//! Endrider Catalyzer — `{1}{R}` 3/1 Human Warrior.
//! Start your engines!
//! Max speed — {T}: Add {R}{R}.
//!
//! "Start your engines!" / "Max speed" are not available KeywordAbility variants
//! and the speed mechanic is not modeled → keywords: vec![] (GAP). The mana
//! ability is GAP'd because it is gated on "Max speed" (speed == 4); there is no
//! speed-precondition for activation, and emitting it ungated would be a
//! materially stronger, always-on mana ability. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Endrider Catalyzer");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![], // GAP: Start your engines! / Max speed not modeled.
        ..Default::default()
    };

    // GAP: "Max speed — {T}: Add {R}{R}" — no Max-speed activation gate.
    reg.register(CardDefinition::new(name, chars))
}
