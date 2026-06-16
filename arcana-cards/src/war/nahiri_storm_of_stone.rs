//! Nahiri, Storm of Stone — `{2}{R/W}{R/W}` Legendary Planeswalker — Nahiri.
//! Colors R, W (the hybrid pips are both red and white). Starting loyalty 5
//! (oracle).
//!
//! Static: During your turn, creatures you control have first strike and
//!     equip abilities you activate cost {1} less to activate.
//!     GAP: a turn-gated continuous static (grant first strike to your
//!     creatures during your turn + equip-cost reduction) is not a loyalty
//!     ability and is not expressible as a loyalty-ability effect.
//! −X: Nahiri deals X damage to target tapped creature.
//!     GAP: dynamic-X loyalty cost — `remove_self_counter` is a fixed u32, so
//!     a "−X" ability cannot be costed; the ability is omitted rather than
//!     mis-costed.
//!
//! No loyalty abilities are expressible (the only two abilities are a
//! turn-gated static and a dynamic-X ability), so the card is registered
//! with bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nahiri, Storm of Stone");
    let nahiri = reg.interner_mut().intern("Nahiri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nahiri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/W}{R/W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
