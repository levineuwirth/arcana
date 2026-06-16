//! Angry Mob — `{2}{W}{W}` Human with Trample.
//! Power/toughness are 2 plus the number of Swamps your opponents
//! control during your turn, and 2 on other turns.
//!
//! GAP: the variable, turn-conditional power/toughness static
//! (a characteristic-defining ability that swaps the base P/T based on
//! whose turn it is and how many Swamps opponents control) is not
//! expressible with the demonstrated effect/static API. The base
//! 2/2 (the floor and the off-turn value) is recorded.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angry Mob");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
