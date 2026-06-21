//! Cheatyface — `{U}{U}{U}` 2/2 Efreet with Flying.
//!
//! Oracle:
//! * If Cheatyface is in your hand, you may sneak Cheatyface onto the
//!   battlefield. If an opponent catches you right away, that player may
//!   exile Cheatyface.
//! * Flying
//!
//! Only Flying is expressible. The "sneak it onto the battlefield" silver-
//! bordered mechanic has no engine representation.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cheatyface");
    let efreet = reg.interner_mut().intern("Efreet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "you may sneak Cheatyface onto the battlefield … that player may
    // exile Cheatyface." — silver-bordered sneak mechanic, no engine hook.

    reg.register(CardDefinition::new(name, chars))
}
