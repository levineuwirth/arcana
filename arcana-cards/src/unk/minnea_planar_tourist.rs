//! Minnea, Planar Tourist — `{3}{G}` 3/4 Legendary Human Scout.
//! * "As Minnea, Planar Tourist enters the battlefield, choose a plane."
//!   — an as-enters replacement choice with no engine notion of a
//!   "plane"; GAP.
//! * "Spells from the chosen plane cost {W}, {U}, {B}, {R}, or {G} less
//!   to cast." — a static cost reduction keyed on the flavor-defined
//!   "plane" of a card; not expressible; GAP.
//!
//! Only the creature bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Minnea, Planar Tourist");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
