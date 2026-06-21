//! Patient Zero — `{1}{B}` 2/2 black Zombie.
//! Lifelink.
//! Damage isn't removed from creatures your opponents control during
//! cleanup steps.
//!
//! Lifelink is wired as a keyword. The persistent-damage static (a
//! cleanup-step replacement that keeps marked damage on opponents'
//! creatures) has no available Effect / replacement primitive in the
//! creature-with-abilities surface, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Patient Zero");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    // GAP: "Damage isn't removed from creatures your opponents control
    // during cleanup steps" is a cleanup-step damage-removal replacement
    // static with no available primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
