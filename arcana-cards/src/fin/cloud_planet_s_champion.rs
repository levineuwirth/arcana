//! Cloud, Planet's Champion — `{3}{R}{W}` 4/4 Legendary Human Soldier Mercenary.
//!
//! Both printed abilities are GAP'd:
//!  * "During your turn, as long as Cloud is equipped, it has double
//!    strike and indestructible" — a conditional (equipped-gated) static
//!    self-grant with no available primitive.
//!  * "Equip abilities you activate that target Cloud cost {2} less to
//!    activate" — an activation cost-reduction static with no available
//!    primitive.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cloud, Planet's Champion");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(mercenary);

    // GAP: equipped-gated static (double strike + indestructible during
    // your turn while equipped).
    // GAP: equip-cost-reduction static ("{2} less to activate").
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
