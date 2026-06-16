//! Serra Avenger — `{W}{W}` 3/3 white Angel with Flying and Vigilance.
//! "You can't cast Serra Avenger during your first, second, or third turns of
//!  the game."
//!
//! Flying + Vigilance are base keywords. The cast-timing restriction is a static
//! casting-permission modifier with no Effect/Trigger representation and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serra Avenger");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "can't cast during your first, second, or third turns" — static casting-permission restriction.
    reg.register(CardDefinition::new(name, chars))
}
