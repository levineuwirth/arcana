//! Rampant Rejuvenator — `{3}{G}` 0/0 green Plant Hydra.
//!
//! * This creature enters with two +1/+1 counters on it.
//! * When this creature dies, search your library for up to X basic land
//!   cards, where X is this creature's power, put them onto the battlefield,
//!   then shuffle.
//!
//! Both abilities are GAP'd: the "enters with N +1/+1 counters" replacement
//! effect has no expressible primitive in the demonstrated API, and the dies
//! trigger fetches a VARIABLE number (up to X basic lands, X = power) of cards
//! onto the battlefield — `Effect::TutorToBattlefield` performs a single fetch
//! with no count parameter, so the "up to X" scaling cannot be expressed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rampant Rejuvenator");
    let plant = reg.interner_mut().intern("Plant");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "enters with two +1/+1 counters" is a replacement effect; no
    // demonstrated primitive expresses ETB-with-counters here.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_fetch_basics,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_fetch_basics(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "search your library for up to X basic land cards, where X is this
    // creature's power, put them onto the battlefield" — a VARIABLE-count fetch
    // to the battlefield. `Effect::TutorToBattlefield` performs a single fetch
    // with no count field, so the dynamic "up to X" cannot be expressed.
    Vec::new()
}
