//! Rankle, Pitiless Trickster — `{1}{B}{B}` Legendary 1/3 Faerie Rogue
//! with Flying. "Rankle has lifelink and haste as long as an opponent
//! controls no creatures." (static, GAP). "When Rankle enters, you may
//! pay 1 life. When you do, each player discards a card and sacrifices
//! a creature." "Whenever a player discards a card, Rankle perpetually
//! gets +1/+0."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rankle, Pitiless Trickster");
    let faerie = reg.interner_mut().intern("Faerie");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "has lifelink and haste as long as an opponent controls no
    // creatures" — conditional continuous static, not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_pay_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: perpetual_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_pay_life(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "each player discards a card and sacrifices a creature" — wrapped in a
    // Sequence so the resolution park loop pauses between each player-choice
    // (the OptionalPayment.then branch is now flattened/parked by the engine).
    let players = script::all_players(state);
    let mut inner: Vec<Effect> = Vec::new();
    for p in players {
        inner.push(Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
        inner.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        });
    }
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Life(1),
        then: Box::new(Effect::Sequence(inner)),
        else_effect: None,
    }]
}

fn perpetual_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // FIDELITY GAP: "perpetually" approximated by a permanent pump.
    vec![Effect::Pump {
        target: trig.source,
        power: 1,
        toughness: 0,
        duration: Duration::Permanent,
        keywords: vec![],
    }]
}
