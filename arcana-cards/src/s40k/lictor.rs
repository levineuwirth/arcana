//! Lictor — `{3}{G}` 3/3 Tyranid with Flash.
//! "Pheromone Trail — When this creature enters, if a creature entered the
//! battlefield under an opponent's control this turn, create a 3/3 green
//! Tyranid Warrior creature token with trample."
//!
//! Flash is a keyword. The ETB ability carries an intervening-if ("if a
//! creature entered the battlefield under an opponent's control this turn")
//! for which no `conditions::` predicate exists; per the intervening-if
//! rule it must not be baked into the effect nor fired unconditionally, so
//! the whole triggered ability is GAP'd. Bones + Flash are faithful.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lictor");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: ETB "if a creature entered under an opponent's control this turn,
    // create a 3/3 green Tyranid Warrior with trample" — no conditions::
    // predicate for "creature entered under opponent's control this turn",
    // so the intervening-if gate cannot be expressed and the trigger is
    // omitted rather than fired unconditionally.
    reg.register(CardDefinition::new(name, chars))
}
