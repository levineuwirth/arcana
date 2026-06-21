//! The Pleasant Taxer — `{1}{W}` 2/2 legendary white Human Wizard.
//!
//! * As The Pleasant Taxer enters the battlefield, choose casting creature
//!   spells, casting noncreature spells, searching libraries, drawing a
//!   card beyond the first each turn, or activating abilities other than
//!   mana abilities.
//! * Doing the chosen action costs players {1} more.
//!
//! GAP (both abilities): the ETB "choose an action category" and the
//! resulting static "doing the chosen action costs {1} more" tax have no
//! expressible primitives (no chosen-category state, no cost-increase
//! effect for this card class); emitted as a vanilla 2/2.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Pleasant Taxer");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
