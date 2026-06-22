//! Thoughtweft Trio — `{2}{W}{W}` 5/5 white Kithkin Soldier with First
//! strike and Vigilance.
//!
//! * First strike, Vigilance — keyword line.
//! * Champion a Kithkin — GAP: the Champion mechanic (ETB "sacrifice unless
//!   you exile another Kithkin you control", plus the leaves-battlefield
//!   return) is not in the usable keyword surface and isn't expressible with
//!   the available effects.
//! * "This creature can block any number of creatures." — GAP: a static
//!   combat-rule modification with no triggered/activated form.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thoughtweft Trio");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
