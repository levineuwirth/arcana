//! Relentless X-ATM092 — `{6}` 6/5 Artifact Creature — Robot Spider.
//! "This creature can't be blocked except by three or more creatures."
//! (blocker-count restriction — GAP'd)
//! "{8}: Return this card from your graveyard to the battlefield
//! tapped with a finality counter on it." (self-reanimate with tapped
//! + finality-counter riders — GAP'd)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Relentless X-ATM092");
    let robot = reg.interner_mut().intern("Robot");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: "can't be blocked except by three or more creatures" — no menace-N / blocker-count static primitive.
    // GAP: "{8}: Return this from your graveyard to the battlefield tapped with a finality counter" —
    //      no self-specific reanimate-with-tapped+finality-counter effect; Reanimate is unfiltered and lacks the riders.
    reg.register(CardDefinition::new(name, chars))
}
