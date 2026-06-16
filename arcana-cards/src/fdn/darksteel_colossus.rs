//! Darksteel Colossus — `{11}` 11/11 Artifact Creature — Golem with
//! Trample and Indestructible. "If Darksteel Colossus would be put
//! into a graveyard from anywhere, reveal it and shuffle it into its
//! owner's library instead." That third line is a replacement effect
//! (CR 614), not a triggered or activated ability, so only the bones +
//! keywords are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Darksteel Colossus");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{11}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(11)),
        toughness: Some(PtValue::Fixed(11)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: "would be put into a graveyard → reveal and shuffle into library instead" — a
    // replacement effect (CR 614), not expressible via triggered/activated ability primitives.
    reg.register(CardDefinition::new(name, chars))
}
