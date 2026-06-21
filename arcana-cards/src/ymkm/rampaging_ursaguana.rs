//! Rampaging Ursaguana — `{4}{G}{G}` 4/4 Creature — Bear Lizard Mutant.
//!
//! Trample, ward {2}, haste.
//! Rampaging Ursaguana gets +2/+2 for each time this creature attacked this
//! game.
//! Disguise {2}{G}{G}.
//!
//! Trample, Haste, and Ward {2} are base keywords. The "+2/+2 for each time it
//! attacked this game" is a STATIC continuous self-buff scaling on a
//! per-game attack count with no script helper to read it — GAP'd. Disguise
//! has no `KeywordAbility` variant — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rampaging Ursaguana");
    let bear = reg.interner_mut().intern("Bear");
    let lizard = reg.interner_mut().intern("Lizard");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(lizard);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
            KeywordAbility::Haste,
        ],
        // GAP: static "+2/+2 for each time this creature attacked this game" —
        // no script helper for a per-game attack count; scaling unexpressible.
        // GAP: Disguise {2}{G}{G} — no KeywordAbility variant.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
