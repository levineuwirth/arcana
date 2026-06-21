//! Retto, Family Racer — `{1}{G}` 2/2 Legendary Human Pilot.
//!
//! * Before the game begins, if this is your commander or in your deck,
//!   secretly write down the names of five creature cards to be part of
//!   your family. (GAP'd — pre-game secret-list mechanic.)
//! * As members of your family enter, you may say "Nothing is stronger
//!   than family" and reveal its name from your list. That creature enters
//!   with two +1/+1 counters on it. (GAP'd — depends on the secret family
//!   list set up before the game; no trigger/effect can express the
//!   per-name membership check.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Retto, Family Racer");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: both abilities rely on a pre-game secret list of five creature
    // names ("your family"); neither the list nor the per-name membership
    // check is expressible with the available trigger/effect API.
    reg.register(CardDefinition::new(name, chars))
}
