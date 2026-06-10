//! Arni Metalbrow — `{2}{R}` 3/3 Legendary Creature — Human Berserker.
//! Whenever a creature you control attacks or enters attacking, you may pay {1}{R}. If you do,
//! you may put a creature card with mana value less than that creature's mana value from your
//! hand onto the battlefield tapped and attacking.
//! Wired: OptionalPayment({1}{R}) wraps Effect::PutOntoBattlefieldTappedAttacking on a
//! creature card from hand whose mana value is less than the attacking creature's (read
//! from the CreatureAttacks trigger event at resolution).
//! GAP: the "you may put" card choice is a deterministic pick (highest-mana-value
//! eligible creature card in hand, taken whenever the mana is paid).
//! GAP: the "enters attacking" half of the trigger condition ("attacks or enters
//! tapped and attacking") is not modeled — only declared attacks fire it.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::GameEvent;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arni Metalbrow");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: arcana_core::targets::ObjectFilter::creature()
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn on_attack(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let you = trig.controller;
    // Mana value of the attacking creature, from the trigger event.
    let GameEvent::CreatureAttacks { attacker, .. } = trig.trigger_event else {
        return Vec::new();
    };
    let Some(attacker_mv) = state
        .objects
        .get(attacker)
        .map(|o| o.characteristics.mana_value())
    else {
        return Vec::new();
    };
    // GAP: "you may put" rendered as a deterministic pick — the
    // highest-mana-value creature card in hand with mv < attacker's mv.
    let chosen = state
        .objects
        .objects_in_zone(Zone::Hand(you))
        .filter(|o| o.is_creature() && o.characteristics.mana_value() < attacker_mv)
        .max_by_key(|o| (o.characteristics.mana_value(), std::cmp::Reverse(o.id)))
        .map(|o| o.id);
    let Some(card) = chosen else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: you,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{R}").expect("valid cost")),
        then: Box::new(Effect::PutOntoBattlefieldTappedAttacking {
            target: card,
            controller: you,
        }),
        else_effect: None,
    }]
}
