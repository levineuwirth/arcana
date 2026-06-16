//! Embodiment of Agonies — `{1}{B}{B}` 0/0 Demon with Flying and Deathtouch.
//! Enters with a +1/+1 counter for each different mana cost among nonland cards
//! in your graveyard. (GAP — count of *distinct* mana costs in graveyard is not
//! expressible with the available script helpers.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Embodiment of Agonies");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // GAP: "enters with a +1/+1 counter for each different mana cost among nonland
    // cards in your graveyard" — no script helper counts distinct mana costs.

    reg.register(CardDefinition::new(name, chars))
}
