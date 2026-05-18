//! Arcbound Wanderer — `{6}` 0/0 Artifact Creature — Golem with Modular—Sunburst.
//! Fifth Dawn uncommon; enters with a +1/+1 counter for each color of
//! mana spent to cast it, and transfers its counters to another
//! artifact creature when it dies.
//!
//! # Rules references
//!
//! * CR 702.44 — Sunburst. This permanent enters with a +1/+1 counter
//!   (creature) or charge counter (non-creature) for each color of mana
//!   spent to cast it. Engine handles ETB counter placement.
//! * CR 702.43 — Modular N. This creature enters with N +1/+1 counters
//!   on it. When it dies, you may put its +1/+1 counters on target
//!   artifact creature. Here Modular works in tandem with Sunburst
//!   (Modular—Sunburst): counters placed by Sunburst are transferred
//!   on death per Modular rules. The N defaults to 1 per convention
//!   since no standalone number is given; Sunburst drives actual ETB
//!   counter count.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Wanderer");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Modular(1), KeywordAbility::Sunburst],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
