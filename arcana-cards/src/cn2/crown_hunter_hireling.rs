//! Crown-Hunter Hireling — `{4}{R}` 4/4 Ogre Mercenary.
//!
//! Oracle:
//! * When this creature enters, you become the monarch.
//! * This creature can't attack unless defending player is the monarch.
//!
//! The monarch designation is not modeled in the engine (no Effect for
//! "become the monarch", no game state for it), so the ETB has no expressible
//! payload and the conditional attack restriction is a GAP'd static.

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
    let name = reg.interner_mut().intern("Crown-Hunter Hireling");
    let ogre = reg.interner_mut().intern("Ogre");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(mercenary);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // When this creature enters, you become the monarch.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: become_monarch,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
    // GAP: "This creature can't attack unless defending player is the monarch"
    // — monarch state is unmodeled; no expressible attack-restriction static.
}

fn become_monarch(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you become the monarch" — the monarch designation is not modeled by
    // any Effect or game state.
    Vec::new()
}
