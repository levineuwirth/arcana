//! Fickle Efreet — `{3}{R}` 5/2 red Efreet.
//! "Whenever this creature attacks or blocks, flip a coin at end of
//! combat. If you lose the flip, an opponent gains control of this
//! creature."
//!
//! GAP: trigger — "attacks or blocks" has no single TriggerCondition
//! variant. We model only the `SelfAttacks` arm; the `SelfBlocks` arm
//! requires a second TriggeredAbilityDef which is beyond the one-trigger
//! shape. The "at end of combat" scheduling is also not captured (the
//! FlipCoin fires at trigger resolution rather than delayed to end of
//! combat).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fickle Efreet");
    let efreet = reg.interner_mut().intern("Efreet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — only SelfAttacks; "or blocks" arm omitted (no combined variant).
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_flip_coin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacks_flip_coin(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // On a loss, an opponent gains control of this creature.
    let opponents = script::opponents(state, trig.controller);
    let opponent = match opponents.first() {
        Some(p) => *p,
        None => return Vec::new(),
    };
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![])),
        lose: Some(Box::new(Effect::ChangeControl {
            target: trig.source,
            new_controller: opponent,
        })),
    }]
}
