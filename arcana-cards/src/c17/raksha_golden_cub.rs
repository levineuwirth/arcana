//! Raksha Golden Cub — `{5}{W}{W}` 3/4 Legendary Cat Soldier with Vigilance.
//!
//! Oracle text:
//! * Vigilance — base keyword.
//! * "As long as Raksha Golden Cub is equipped, Cat creatures you control
//!   get +2/+2 and have double strike." — a conditional static anthem with
//!   no triggered/activated form; not expressible, GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Raksha Golden Cub");
    let cat = reg.interner_mut().intern("Cat");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "As long as ~ is equipped, Cat creatures you control get
    // +2/+2 and have double strike" — equipped-conditional anthem with no
    // expressible static/trigger/activated form; omitted.

    reg.register(CardDefinition::new(name, chars))
}
