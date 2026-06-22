//! Shivan Devastator — `{X}{R}` 0/0 Creature — Dragon Hydra.
//!
//! Oracle:
//! * Flying, haste.
//! * This creature enters with X +1/+1 counters on it.
//!
//! Decomposition: Flying + Haste keywords. The "enters with X +1/+1 counters"
//! line has no expressible effect.
//!
//! GAP: "enters with X +1/+1 counters" depends on the spell's cast X value,
//!      which is not exposed to an ETB trigger (cf. Apocalypse Hydra) — no
//!      expressible effect.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shivan Devastator");
    let dragon = reg.interner_mut().intern("Dragon");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
