//! Alluring Suitor // Deadly Dancer
//!
//! Front face: Creature — Vampire {2}{R}, 2/3, red.
//! "When you attack with exactly two creatures, transform this creature."
//! GAP: 'exactly two creatures attacking' trigger condition not expressible.
//!
//! Back face: Creature — Vampire, red.
//! Trample.
//! "When this creature transforms into Deadly Dancer, add {R}{R}. Until end
//! of turn, you don't lose this mana as steps and phases end."
//! GAP: 'until end of turn, don't lose mana as steps/phases end' not modeled.
//! Mana addition on transform is modeled via TriggerCondition::Transform.
//! GAP: back-face-only triggered ability not modeled.
//!
//! "{R}{R}: This creature and another target creature each get +1/+0 until
//! end of turn." — Activated ability, GAP: back-face-only activated ability
//! not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alluring Suitor");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Deadly Dancer");
    let vampire_back_sub = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
        // GAP: 'when you attack with exactly two creatures' trigger not modeled
        // GAP: back-face on-transform mana add and 'don't lose mana as phases end' not modeled
        // GAP: back-face activated ability {R}{R}: pump two creatures not modeled
    )
}
