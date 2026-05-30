//! Ragged Recluse // Odious Witch — `{1}{B}` Creature — Human Peasant 2/1 (front) /
//! Creature — Human Warlock (back). Transform.
//!
//! Front: At the beginning of your end step, if you discarded a card this turn, transform.
//! Back: Whenever this creature attacks, defending player loses 1 life and you gain 1 life.
//!
//! GAP: "if you discarded a card this turn" intervening-if — wired as an inline check in the
//!   effect fn using script::cards_discarded_this_turn.
//! GAP: Back-face attack trigger fires on both faces (no face-gate on TriggeredAbilityDef).
//! GAP: "defending player" for the attack trigger is approximated via trig.defending_player().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ragged Recluse");

    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Odious Witch");
    let back_human_sub = reg.interner_mut().intern("Human");
    let warlock_sub = reg.interner_mut().intern("Warlock");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(warlock_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: front face — at beginning of your end step, if you discarded a card this turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2: back face — whenever this creature attacks, drain 1 from defending player.
            // GAP: back-face-only; fires on both faces.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn end_step_transform(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let discarded = script::cards_discarded_this_turn(state, trig.controller);
    if discarded > 0 {
        vec![Effect::Transform { target: trig.source }]
    } else {
        Vec::new()
    }
}

fn attack_drain(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: back-face-only.
    let defending = trig.defending_player().unwrap_or(trig.controller);
    vec![
        Effect::LoseLife { player: defending, amount: 1 },
        Effect::GainLife { player: trig.controller, amount: 1 },
    ]
}
