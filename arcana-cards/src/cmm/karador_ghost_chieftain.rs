//! Karador, Ghost Chieftain — `{5}{W}{B}{G}` 3/4 Legendary Centaur Spirit.
//! "This spell costs {1} less to cast for each creature card in your
//!  graveyard. Once during each of your turns, you may cast a creature
//!  spell from your graveyard."
//!
//! Both abilities are pure statics that the demonstrated primitives can't
//! express:
//! GAP: cost reduction "{1} less for each creature card in your graveyard"
//!      — no cost-modification primitive in the engine API.
//! GAP: "Once during each of your turns, you may cast a creature spell
//!      from your graveyard" — no cast-from-graveyard permission primitive.
//! Emitting bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karador, Ghost Chieftain");
    let centaur = reg.interner_mut().intern("Centaur");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
