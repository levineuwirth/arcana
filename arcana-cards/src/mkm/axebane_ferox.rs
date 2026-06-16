//! Axebane Ferox — `{2}{G}{G}` 4/4 Beast with Deathtouch and Haste.
//! "Ward—Collect evidence 4."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Axebane Ferox");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Ward—Collect evidence 4" is a non-mana ward cost, which is not
        // expressible (KeywordAbility::Ward takes a ManaCost only).
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
