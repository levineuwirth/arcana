//! Vile Manifestation — `{1}{B}` 0/4 Horror (B).
//! This creature gets +1/+0 for each card with cycling in your graveyard.
//! (GAP — dynamic static power boost.)
//! Cycling {2}.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vile Manifestation");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost"))],
        // GAP: "+1/+0 for each card with cycling in your graveyard" — dynamic
        // static power boost not expressible.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
