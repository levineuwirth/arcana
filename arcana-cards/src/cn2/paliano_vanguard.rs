//! Paliano Vanguard — `{1}{W}` 2/2 Human Soldier. Draft-matters card:
//! "Draft this card face up. As you draft a creature card, you may
//! reveal it, note its creature types, then turn this card face down.
//! Other creatures you control of a type you noted ... get +1/+1."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paliano Vanguard");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: draft-matters text — "Draft this card face up", "As you draft a
    // creature card ... note its creature types", and the noted-type static
    // "+1/+1" anthem are draft-phase mechanics with no engine modeling and
    // no triggered/activated ability surface.
    reg.register(CardDefinition::new(name, chars))
}
