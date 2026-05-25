//! Skullslither Worm — `{3}{B}` 3/3 black Creature — Worm.
//! "When this creature enters, each opponent discards a card. For each
//! opponent who can't, put two +1/+1 counters on this creature."
//!
//! GAP: conditional counters for each opponent who can't discard —
//! the engine has no Effect variant that tracks "couldn't discard"
//! per opponent. Emitting the discard for each opponent; GAP the
//! conditional counter placement.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skullslither Worm");
    let worm = reg.interner_mut().intern("Worm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(worm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_etb(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let mut effects: Vec<Effect> = opponents.into_iter().map(|p| {
        Effect::Discard { player: p, count: 1, choice: DiscardChoice::ControllerChooses }
    }).collect();
    // GAP: for each opponent who can't discard, put two +1/+1 counters on
    // this creature — conditional counter based on "couldn't discard" is
    // not expressible with the catalog.
    effects
}
