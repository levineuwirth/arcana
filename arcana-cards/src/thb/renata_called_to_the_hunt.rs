//! Renata, Called to the Hunt — `{2}{G}{G}` */3 Legendary Enchantment
//! Creature — Demigod. Power equals your devotion to green (a CDA), and
//! each other creature you control enters with an extra +1/+1 counter.
//! Both are static abilities outside the demonstrated trigger/activated
//! surface.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Renata, Called to the Hunt");
    let demigod = reg.interner_mut().intern("Demigod");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demigod);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "Renata's power is equal to your devotion to green" — characteristic-
    //      defining static; no static-CDA Effect/wiring in this surface.
    // GAP: "Each other creature you control enters with an additional +1/+1
    //      counter on it" — enters-with replacement static; not expressible.
    reg.register(CardDefinition::new(name, chars))
}
