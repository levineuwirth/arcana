//! Mist Dancer — `{4}{U}` 3/3 Merfolk Wizard with Flying.
//! "Flying. Other Merfolk you control get +1/+0 and have flying. Encore
//! {5}{U}{U}."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mist Dancer");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Other Merfolk you control get +1/+0 and have flying" — no
    // static-anthem primitive available here.
    // GAP: Encore {5}{U}{U} — Encore is not a usable KeywordAbility variant and
    // there is no graveyard-activation primitive for the token-copy mechanic.
    reg.register(CardDefinition::new(name, chars))
}
