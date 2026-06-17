//! Heedless One — `{3}{G}` */* Elf Avatar with Trample.
//! "Heedless One's power and toughness are each equal to the number of Elves on
//! the battlefield." (Characteristic-defining ability.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heedless One");
    let elf = reg.interner_mut().intern("Elf");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // */* — printed star power/toughness.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: the characteristic-defining ability "power and toughness each equal to
    // the number of Elves on the battlefield" is a continuous CDA static, not a
    // triggered/activated ability — not expressible with the demonstrated
    // MultiAbilityCreature API. The */* bones are recorded via PtValue::Star.
    reg.register(CardDefinition::new(name, chars))
}
