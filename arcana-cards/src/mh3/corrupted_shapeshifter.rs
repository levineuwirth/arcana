//! Corrupted Shapeshifter — `{3}{U}` */* Eldrazi Shapeshifter, Devoid
//! (colorless). "As this creature enters, it becomes your choice of a 3/3 with
//! flying, a 2/5 with vigilance, or a 0/12 with defender." (as-enters
//! replacement choice — GAP)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Corrupted Shapeshifter");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(shapeshifter);

    // GAP: "As this creature enters, it becomes your choice of a 3/3 with
    // flying, a 2/5 with vigilance, or a 0/12 with defender." — an as-enters
    // PLAYER-CHOICE replacement that fixes P/T and grants a keyword. The
    // self-CDA constructors resolve a computed/counted `*`, not a one-of-three
    // player choice, so this is not expressible. P/T emitted as PtValue::Star
    // bones.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
