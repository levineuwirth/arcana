//! Tinybones, Trinket Thief — `{1}{B}` 1/2 Legendary Skeleton Rogue.
//! "At the beginning of each end step, if an opponent discarded a card
//! this turn, you draw a card and you lose 1 life. {4}{B}{B}: Each
//! opponent with no cards in hand loses 10 life."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tinybones, Trinket Thief");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_opponent_discarded),
                effect: draw_and_lose,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{B}{B}: Each opponent with no cards in hand loses 10 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: empty_handed_opponents_lose_10,
            }),
    )
}

fn if_opponent_discarded(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::opponents(s, you)
        .into_iter()
        .any(|opp| script::cards_discarded_this_turn(s, opp) >= 1)
}

fn draw_and_lose(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}

fn empty_handed_opponents_lose_10(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::opponents(state, ctx.controller)
        .into_iter()
        .filter(|&opp| script::hand_size(state, opp) == 0)
        .map(|opp| Effect::LoseLife { player: opp, amount: 10 })
        .collect()
}
