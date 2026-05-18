//! Force of Savagery — `{2}{G}` 8/0 Elemental with Trample.
//! A creature of extreme offensive power and zero toughness; it dies
//! immediately as a state-based action unless toughness is boosted.
//!
//! # Rules references
//!
//! * CR 702.19 — Trample. If this creature would assign damage, it must
//!   assign at least lethal damage to each blocker before the rest can
//!   trample through to the defending player or planeswalker.
//! * CR 704.5f — State-based action: a creature with toughness 0 or less
//!   is put into its owner's graveyard; this creature enters with 0
//!   toughness and is immediately destroyed unless its toughness is raised.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Force of Savagery");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
