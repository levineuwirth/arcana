//! Skyward Spider — `{W/U}{W/U}` 2/2 Spider Human Hero.
//!
//! "Ward {2}
//!  This creature has flying as long as it's modified."
//!
//! Decomposition: Ward {2} keyword. The conditional static "has flying
//! as long as it's modified" is a continuous ability with no engine
//! primitive (there is no "is modified" predicate or conditional keyword
//! grant), so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyward Spider");
    let spider = reg.interner_mut().intern("Spider");
    let human = reg.interner_mut().intern("Human");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(human);
    subtypes.0.insert(hero);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W/U}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };
    // GAP: "This creature has flying as long as it's modified." — no
    // engine primitive for an "is modified" conditional keyword grant.
    reg.register(CardDefinition::new(name, chars))
}
