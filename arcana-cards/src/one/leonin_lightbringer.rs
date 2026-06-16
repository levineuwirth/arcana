//! Leonin Lightbringer — `{2}{W}` 3/2 Cat Rebel with Ward {2}.
//! "Ward {2}. As long as this creature is equipped, it gets +1/+1."
//!
//! Ward {2} is a parametrized keyword. The "as long as equipped, +1/+1"
//! conditional static buff has no triggered/activated form — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leonin Lightbringer");
    let cat = reg.interner_mut().intern("Cat");
    let rebel = reg.interner_mut().intern("Rebel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(rebel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    // GAP: "As long as this creature is equipped, it gets +1/+1" — conditional
    // continuous static, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
