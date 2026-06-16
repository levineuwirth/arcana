//! Dirgur Nemesis — `{5}{U}` 6/5 blue Serpent with Defender and Megamorph {6}{U}.
//!
//! Defender is an expressible keyword. Megamorph (the face-down cast + turn-face-up
//! with a +1/+1 counter) is not in the usable keyword surface, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dirgur Nemesis");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Defender],
        // GAP: Megamorph {6}{U} — face-down cast + morph-flip mechanic not in usable keyword surface.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
