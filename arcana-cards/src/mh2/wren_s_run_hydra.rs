//! Wren's Run Hydra — `{X}{G}` 0/0 Hydra.
//! Reach.
//! This creature enters with X +1/+1 counters on it. (GAP — no documented
//! enters-with-X primitive in the available surface.)
//! Reinforce X—{X}{G}{G}. (GAP — Reinforce is not a usable KeywordAbility, and
//! an X-scaled discard-from-hand activation is not expressible.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wren's Run Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    // GAP: "This creature enters with X +1/+1 counters on it." — no documented
    // enters-with-X primitive in the available effect/keyword surface.
    // GAP: "Reinforce X—{X}{G}{G}" — Reinforce is not a usable KeywordAbility,
    // and an X-scaled discard-from-hand counter activation is not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
