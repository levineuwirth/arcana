//! Zimone and Dina — `{B}{G}{U}` 3/4 Legendary Human Dryad.
//!
//! * Whenever you draw your second card each turn, target opponent
//!   loses 2 life and you gain 2 life.
//! * `{T}, Sacrifice another creature:` Draw a card. You may put a land
//!   card from your hand onto the battlefield tapped. (The "if you
//!   control eight or more lands, repeat this process once" rider is
//!   GAP'd — no repeat-a-resolution primitive.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zimone and Dina");
    let human = reg.interner_mut().intern("Human");
    let dryad = reg.interner_mut().intern("Dryad");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(dryad);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: Some(if_drew_second_card),
                effect: drain_two,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice another creature: Draw a card. You may put a land card from your hand onto the battlefield tapped. If you control eight or more lands, repeat this process once.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_and_land,
            }),
    )
}

fn if_drew_second_card(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    script::cards_drawn_this_turn(s, you) == 2
}

fn drain_two(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife { player: *p, amount: 2 },
        Effect::GainLife { player: trig.controller, amount: 2 },
    ]
}

fn draw_and_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "If you control eight or more lands, repeat this process once" — no repeat-resolution primitive.
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::PutFromHandOntoBattlefield {
            player: ctx.controller,
            filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
    ]
}
