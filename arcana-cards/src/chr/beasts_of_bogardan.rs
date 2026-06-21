//! Beasts of Bogardan — `{4}{R}` 3/3 Beast.
//!
//! Protection from red
//! This creature gets +1/+1 as long as an opponent controls a nontoken
//! white permanent.
//!
//! Protection is not an available `KeywordAbility` variant for this card
//! class, and the conditional static +1/+1 is a pure continuous static with
//! no triggered/activated form. Both are GAP'd; the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beasts of Bogardan");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    // GAP: keyword — Protection from red is not an available KeywordAbility.
    // GAP: static — "+1/+1 as long as an opponent controls a nontoken white
    //      permanent" is a pure conditional continuous static.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
