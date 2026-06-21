//! Snow Villiers — `{2}{W}` */3 Legendary Creature — Human Rebel Monk
//! with Vigilance.
//! "Snow Villiers's power is equal to the number of creatures you
//! control."
//!
//! Vigilance is a base keyword. The `*` power is recorded as
//! PtValue::Star. The characteristic-defining ability that sets power
//! equal to the number of creatures you control is a pure static with
//! no usable Effect/ability hook here — GAP'd as a doc note (it would
//! require a CDA, which this card class can't express).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snow Villiers");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(monk);

    // GAP (static CDA): "Snow Villiers's power is equal to the number of
    // creatures you control." No characteristic-defining-ability hook is
    // available in this card class; power is recorded as `*` (Star) only.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
