//! Lim-Dûl's Paladin — `{2}{B}{R}` 0/3 Human Knight.
//! Trample.
//! "At the beginning of your upkeep, you may discard a card. If you
//! don't, sacrifice this creature and draw a card." (GAP — discard-or-
//! else gate not expressible.)
//! "Whenever this creature becomes blocked, it gets +6/+3 until end of
//! turn."
//! "Whenever this creature attacks and isn't blocked, it assigns no
//! combat damage this turn and defending player loses 4 life." (combat-
//! damage suppression GAP'd; the 4 life loss is wired.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lim-Dûl's Paladin");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "you may discard a card; if you don't, sacrifice this
            // creature and draw a card" — OptionalPayment supports only
            // Mana/Life costs, not a discard-or-else gate. Upkeep trigger
            // fires but resolves to nothing.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_discard_or_else,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: blocked_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacksUnblocked,
                intervening_if: None,
                effect: unblocked_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_discard_or_else(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: discard-a-card-or-else (sacrifice + draw) not expressible.
    Vec::new()
}

fn blocked_pump(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: trig.source,
        power: 6,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn unblocked_drain(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "it assigns no combat damage this turn" — combat-damage
    // suppression for an attacker is not expressible. The 4 life loss
    // to the defending player IS wired.
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    vec![Effect::LoseLife {
        player: p,
        amount: 4,
    }]
}
