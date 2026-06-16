//! Rabid Wombat — `{2}{G}{G}` 0/1 Wombat.
//! "Vigilance. This creature gets +2/+2 for each Aura attached to it."
//!
//! Vigilance is a base keyword. The dynamic self-buff "+2/+2 for each Aura
//! attached to it" is a static continuous ability with no trigger or activation
//! cost — not expressible as a TriggeredAbilityDef / ActivatedAbilityDef in
//! this card class, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rabid Wombat");
    let wombat = reg.interner_mut().intern("Wombat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wombat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        // GAP: static "gets +2/+2 for each Aura attached to it".
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
