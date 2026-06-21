//! Solaflora, Intergalactic Icon — `{3}{W}{W}` 3/3 Legendary Human
//! Guest.
//!
//! Oracle (both lines are static, rule-altering continuous abilities —
//! no trigger word, no activation cost — so both are GAP'd):
//! * GAP: Auras and Equipment you control attached to Solaflora affect
//!   other creatures you control as though they were attached to them.
//! * GAP: Counters and stickers on Solaflora affect other creatures you
//!   control as though those counters and stickers were on them.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Solaflora, Intergalactic Icon");
    let human = reg.interner_mut().intern("Human");
    let guest = reg.interner_mut().intern("Guest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(guest);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
