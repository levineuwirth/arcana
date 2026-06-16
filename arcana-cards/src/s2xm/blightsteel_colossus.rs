//! Blightsteel Colossus — `{12}` 11/11 Artifact Phyrexian Golem with Trample,
//! Infect, and Indestructible. If it would be put into a graveyard from
//! anywhere, reveal it and shuffle it into its owner's library instead.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blightsteel Colossus");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{12}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(11)),
        toughness: Some(PtValue::Fixed(11)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Infect,
            KeywordAbility::Indestructible,
        ],
        ..Default::default()
    };

    // GAP: "If Blightsteel Colossus would be put into a graveyard from anywhere,
    // reveal it and shuffle it into its owner's library instead" — a
    // zone-change replacement effect, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
