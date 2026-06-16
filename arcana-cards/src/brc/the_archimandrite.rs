//! The Archimandrite — `{2}{U}{R}{W}` 0/5 Legendary Human Advisor.
//! At the beginning of your upkeep, you gain X life, where X is the number
//! of cards in your hand minus 4.
//! Whenever you gain life, each Advisor, Artificer, and Monk you control
//! gains vigilance and gets +X/+0 until end of turn, where X is the amount
//! of life you gained.
//! Tap three untapped Advisors, Artificers, and/or Monks you control: Draw a card.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Archimandrite");
    let human = reg.interner_mut().intern("Human");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(advisor);

    // Subtype-OR filter for the tap cost / life-gain payoff: Advisor/Artificer/Monk.
    let advisor2 = reg.interner_mut().intern("Advisor");
    let artificer = reg.interner_mut().intern("Artificer");
    let monk = reg.interner_mut().intern("Monk");
    let class_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![advisor2, artificer, monk]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::LifeGained {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap three untapped Advisors, Artificers, and/or Monks you control: Draw a card.".into(),
                cost: ActivationCost {
                    tap_other: Some(class_filter),
                    tap_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_a_card,
            }),
    )
}

fn upkeep_gain_life(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // X = cards in hand minus 4 (clamp to 0).
    let hand = script::hand_size(state, trig.controller);
    let x = hand.saturating_sub(4);
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: trig.controller, amount: x }]
}

fn on_gain_life(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Grant vigilance to each Advisor/Artificer/Monk you control.
    // GAP: "+X/+0 where X is the amount of life gained" — no accessor for the
    // amount of life gained on a LifeGained trigger, so the pump is omitted.
    let advisor = reg.interner().lookup("Advisor").unwrap_or_default();
    let artificer = reg.interner().lookup("Artificer").unwrap_or_default();
    let monk = reg.interner().lookup("Monk").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![advisor, artificer, monk]);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        })
        .collect()
}

fn draw_a_card(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
