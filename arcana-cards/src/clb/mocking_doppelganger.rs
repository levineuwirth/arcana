//! Mocking Doppelganger — `{3}{U}` 0/0 blue Shapeshifter.
//! Flash.
//! You may have this creature enter as a copy of a creature an opponent
//! controls, except it has "Other creatures with the same name as this
//! creature are goaded."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mocking Doppelganger");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flash],
        // GAP: "You may have this creature enter as a copy of a creature an
        // opponent controls, except it has [a granted goad ability]." There is
        // no demonstrated as-enters copy replacement (Effect::CopyPermanent
        // mints a separate token; it does not make THIS object enter as a
        // copy), nor a way to bolt the goad rider onto the copied result.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
