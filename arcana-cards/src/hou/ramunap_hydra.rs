//! Ramunap Hydra — `{3}{G}` 3/3 green Snake Hydra.
//! Vigilance, reach, trample.
//! "This creature gets +1/+1 as long as you control a Desert."
//! "This creature gets +1/+1 as long as there is a Desert card in your graveyard."
//!
//! The keyword line maps cleanly to base `KeywordAbility` variants. Both
//! remaining lines are pure STATIC continuous self-pump abilities gated on a
//! board / graveyard condition — there is no triggered or activated ability to
//! decompose them into and no expressible static-pump Effect in this surface,
//! so they are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ramunap Hydra");
    let snake = reg.interner_mut().intern("Snake");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Reach,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    // GAP: static "gets +1/+1 as long as you control a Desert" — conditional
    //      continuous self-pump, no expressible Effect in this surface.
    // GAP: static "gets +1/+1 as long as there is a Desert card in your
    //      graveyard" — same.
    reg.register(CardDefinition::new(name, chars))
}
