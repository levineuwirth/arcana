//! Jetmir, Nexus of Revels — `{1}{R}{G}{W}` 5/4 Legendary Cat Demon.
//!
//! Oracle:
//!  * Creatures you control get +1/+0 and have vigilance as long as you control
//!    three or more creatures.
//!  * Creatures you control also get +1/+0 and have trample as long as you
//!    control six or more creatures.
//!  * Creatures you control also get +1/+0 and have double strike as long as you
//!    control nine or more creatures.
//!
//! All three lines are conditional static anthem effects (continuous layer
//! buffs gated on creature count), not triggered/activated abilities — all
//! GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jetmir, Nexus of Revels");
    let cat = reg.interner_mut().intern("Cat");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(demon);

    // GAP: three conditional static anthem effects ("creatures you control get
    // +1/+0 and have <kw> as long as you control N+ creatures") — continuous
    // layer buffs, not triggered/activated abilities.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
