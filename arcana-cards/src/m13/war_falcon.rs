//! War Falcon — `{W}` 2/1 Creature — Bird with Flying.
//! "This creature can't attack unless you control a Knight or a Soldier."
//!
//! Only the Flying keyword is expressible; the conditional attack
//! restriction is a static continuous ability with no triggered/activated
//! decomposition.

// GAP: "This creature can't attack unless you control a Knight or a
//      Soldier." — a static attack-restriction continuous ability, not
//      expressible as a triggered or activated ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("War Falcon");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
