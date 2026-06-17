//! Boneyard Mycodrax — `{2}{B}` */* Fungus with Scavenge {4}{B}.
//! "Boneyard Mycodrax's power and toughness are each equal to the number of
//! other creature cards in your graveyard."
//! Scavenge {4}{B} (engine synthesizes the graveyard activation from the keyword).
//!
//! GAP: the */* characteristic-defining ability (P/T = other creature cards in
//!      your graveyard) is a pure static, not expressible as a triggered/activated
//!      ability; P/T are left as the unresolved star CDA placeholder.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boneyard Mycodrax");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Scavenge(
            ManaCost::parse("{4}{B}").expect("valid cost"),
        )],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
