//! Young Deathclaws — `{2}{B}{G}` 4/2 Lizard Mutant with Menace.
//! "Each creature card in your graveyard has scavenge. The scavenge
//! cost is equal to its mana cost."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Young Deathclaws");
    let lizard = reg.interner_mut().intern("Lizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(mutant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };
    // GAP: static "Each creature card in your graveyard has scavenge" — a
    // continuous ability that grants scavenge to other cards; no engine
    // primitive grants an activated ability to cards in another zone.
    reg.register(CardDefinition::new(name, chars))
}
