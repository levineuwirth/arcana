//! The Cookout Creator — `{W}{B}{G}` 3/3 Legendary Human Gamer
//! (white/black/green).
//!
//! * "When The Cookout Creator enters, create a Food token." —
//!   implemented.
//! * "At the beginning of your upkeep, create a 1/1 colorless Human
//!   creature token for each Food you control." — implemented (one
//!   token per Food via ForEach over the matching ids).
//! * "Tap four untapped Humans you control: You draw a card and create
//!   a Treasure token." — implemented as a tap-four-Humans cost.

use arcana_core::effects::{CommodityToken, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Cookout Creator");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let human_filter = script::subtype_filter(reg, "Human")
        .controlled_by(ControllerConstraint::You)
        .untapped_only();

    reg.register(
        CardDefinition::new(name, chars)
            // "When The Cookout Creator enters, create a Food token."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "At the beginning of your upkeep, create a 1/1 colorless
            // Human creature token for each Food you control."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_make_humans,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Tap four untapped Humans you control: You draw a card and
            // create a Treasure token."
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap four untapped Humans you control: You draw a card \
                       and create a Treasure token."
                    .into(),
                cost: ActivationCost {
                    tap_other: Some(human_filter),
                    tap_other_count: 4,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_and_treasure,
            }),
    )
}

fn etb_make_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}

fn upkeep_make_humans(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let food_filter = script::subtype_filter(reg, "Food")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &food_filter, trig.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    let human = reg.interner().lookup("Human").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token,
        }),
    }]
}

fn draw_and_treasure(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::CreateCommodityToken {
            controller: ctx.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        },
    ]
}
