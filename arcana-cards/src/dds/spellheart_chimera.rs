//! Spellheart Chimera — `{1}{U}{R}` */3 Chimera with Flying and Trample.
//! "Spellheart Chimera's power is equal to the number of instant and sorcery
//!  cards in your graveyard."
//!
//! Power is a characteristic-defining ability, emitted as `PtValue::Star`;
//! the actual count-instants/sorceries-in-graveyard CDA is an engine concern
//! (no per-card hook) and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spellheart Chimera");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chimera);

    // GAP: "power is equal to the number of instant and sorcery cards in your
    // graveyard" — CDA emitted as PtValue::Star; the count is an engine concern.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
