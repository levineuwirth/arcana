//! Blue, Loyal Raptor — `{2}{G}{U}` 5/4 Legendary Dinosaur.
//!
//! * "Partner with Owen Grady, Raptor Trainer (When this creature enters, target player
//!   may put Owen into their hand from their library, then shuffle.)" GAP: Partner with
//!   is not a KeywordAbility variant; the optional fetch-by-name-for-a-target-player ETB
//!   is not expressible.
//! * "For each kind of counter on Blue, Loyal Raptor, each other Dinosaur you control
//!   enters with a counter of that kind on it." GAP: enters-with-counters replacement
//!   static keyed on this permanent's counters is not expressible.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blue, Loyal Raptor");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
