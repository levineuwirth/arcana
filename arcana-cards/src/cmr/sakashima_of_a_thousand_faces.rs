//! Sakashima of a Thousand Faces — `{3}{U}` 3/1 Legendary Creature —
//! Human Rogue.
//!
//! All printed abilities are inexpressible with the available surface:
//! * "You may have Sakashima enter as a copy of another creature you
//!   control, except it has Sakashima's other abilities." — an
//!   as-enters copy replacement; no primitive.
//! * "The 'legend rule' doesn't apply to permanents you control." —
//!   a rule-altering static; no primitive.
//! * Partner — not in the usable keyword surface.
//! Emitted as faithful bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sakashima of a Thousand Faces");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: enter-as-a-copy replacement; legend-rule-doesn't-apply
    // static; Partner keyword — none expressible.

    reg.register(CardDefinition::new(name, chars))
}
