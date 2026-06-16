//! Drana and Linvala — `{1}{W}{W}{B}` Legendary 3/4 Vampire Angel (B/W).
//!
//! Rules text:
//! * Flying, vigilance (keyword line).
//! * Activated abilities of creatures your opponents control can't be
//!   activated. (static lockdown — GAP)
//! * Drana and Linvala has all activated abilities of all creatures your
//!   opponents control. You may spend mana as though it were mana of any
//!   color to activate those abilities. (static ability-grant — GAP)
//!
//! Both non-keyword clauses are pure STATIC continuous abilities (no trigger
//! word, no activation cost): a global "can't be activated" restriction and an
//! ability-stealing static. Neither is a triggered or activated ability and
//! neither is expressible with the demonstrated Effect API, so both are GAP'd.
//! The keyword line is fully expressed.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drana and Linvala");
    let vampire = reg.interner_mut().intern("Vampire");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: "Activated abilities of creatures your opponents control can't be
    //       activated." — static activation lockdown, not expressible.
    // GAP: "Drana and Linvala has all activated abilities of all creatures your
    //       opponents control. You may spend mana as though it were any color
    //       to activate them." — static ability-grant + mana substitution, not
    //       expressible.

    reg.register(CardDefinition::new(name, chars))
}
