//! Phyrexian Broodstar — `{6}{U}{U}` */* Phyrexian Beast with Flying.
//! "Affinity for Phyrexians (This spell costs {1} less to cast for each
//!  Phyrexian you control.)
//!  Flying
//!  Phyrexian Broodstar's power and toughness are each equal to the number
//!  of Phyrexians you control."
//!
//! Flying is a keyword. Affinity is a cost-reduction keyword with no
//! expressible variant — GAP. The characteristic-defining "power and
//! toughness each equal to the number of Phyrexians you control" has no
//! demonstrated CDA registration hook, so P/T is left as `*`
//! (PtValue::Star) and the computing static is GAP'd.

// GAP (CDA): "power and toughness are each equal to the number of
// Phyrexians you control" — no registration hook for a characteristic-
// defining power/toughness static.
// GAP (keyword): Affinity for Phyrexians — cost reduction not expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Broodstar");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
