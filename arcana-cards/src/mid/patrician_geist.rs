//! Patrician Geist — `{2}{U}` 2/2 Spirit Knight with Flying.
//!
//! Oracle:
//! * Flying — base keyword.
//! * "Other Spirits you control get +1/+1." A pure static continuous anthem;
//!   no demonstrated triggered/activated primitive expresses a standing
//!   subtype-scoped +1/+1 boost. GAP.
//! * "Spells you cast from your graveyard cost {1} less to cast." A static
//!   cost reduction; no demonstrated primitive expresses a cast-from-graveyard
//!   cost reduction. GAP.
//!
//! Only the Flying keyword is expressible with the demonstrated API; both
//! statics are noted as GAPs.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patrician Geist");
    let spirit = reg.interner_mut().intern("Spirit");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static — "Other Spirits you control get +1/+1." (subtype-scoped
    // anthem, no demonstrated static-continuous primitive).
    // GAP: static — "Spells you cast from your graveyard cost {1} less to
    // cast." (zone-scoped cost reduction, no demonstrated primitive).
    reg.register(CardDefinition::new(name, chars))
}
