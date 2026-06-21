//! Zoanthrope — `{X}{U}{R}` 0/0 Creature — Tyranid.
//! Ravenous (enters with X +1/+1 counters; if X >= 5 draw a card on ETB).
//! Flying, ward {2}.
//! Warp Blast — When this creature enters, it deals X damage to any target.
//!
//! Bones + Flying + Ward {2} are wired. The Ravenous keyword (X +1/+1
//! counters on ETB, conditional draw) has no engine variant and is GAP'd.
//! Warp Blast's ETB X damage is also GAP'd: X here is the spell's cast-time
//! X, and there is no accessor exposing the resolved X to a trigger effect
//! fn (the dynamic-X resolver only stamps `TargetCount::X` from trigger
//! EVENT data, not from the cast). Emitting a fixed/0 amount would be a
//! materially wrong card, so the damage clause is omitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::effects::KeywordAbility;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zoanthrope");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: Ravenous (enters with X +1/+1 counters; if X >= 5 draw a card)
        // — no Ravenous keyword variant and X-counter-on-ETB is not expressible.
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: Warp Blast — "When this creature enters, it deals X damage to any
    // target." X is the spell's cast-time X; no accessor exposes a resolved
    // cast-X to a trigger effect fn, so the damage amount is inexpressible.
    reg.register(CardDefinition::new(name, chars))
}
