//! Guardian Augmenter — `{2}{G}` 2/2 Troll Wizard.
//! Flash.
//! Commander creatures you control get +2/+2.
//! Commanders you control have hexproof.
//!
//! Flash is a base keyword. The two commander-anchored static continuous
//! abilities are GAP'd: there is no commander-typed ObjectFilter predicate
//! or static anthem/keyword-grant primitive in the demonstrated API.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Guardian Augmenter");
    let troll = reg.interner_mut().intern("Troll");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: "Commander creatures you control get +2/+2" — static anthem over a
    //      commander-typed filter (no commander predicate / static pump in API).
    // GAP: "Commanders you control have hexproof" — static keyword grant over
    //      commanders (same limitation).
    reg.register(CardDefinition::new(name, chars))
}
