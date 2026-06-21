//! Nameless One — `{3}{U}` */* Wizard Avatar.
//!
//! Oracle:
//! * "Nameless One's power and toughness are each equal to the number
//!   of Wizards on the battlefield." — a characteristic-defining
//!   ability; P/T are marked as `*` (PtValue::Star). The CDA that sets
//!   the actual value is a continuous static and is GAP'd (no
//!   triggered/activated form).
//! * "Morph {2}{U}." — the morph keyword with its mana cost.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nameless One");
    let wizard = reg.interner_mut().intern("Wizard");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wizard);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Morph(
            ManaCost::parse("{2}{U}").expect("valid cost"),
        )],
        ..Default::default()
    };

    // GAP: CDA "power and toughness are each equal to the number of
    // Wizards on the battlefield" — a continuous characteristic-defining
    // static, not a triggered/activated ability (P/T marked `*`).
    reg.register(CardDefinition::new(name, chars))
}
