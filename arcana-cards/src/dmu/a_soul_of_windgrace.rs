//! A-Soul of Windgrace — `{1}{B}{R}{G}` 5/4 Legendary Cat Avatar.
//!
//! Oracle:
//! * "Whenever Soul of Windgrace enters or attacks, you may put a land
//!   card from a graveyard onto the battlefield tapped under your
//!   control." — decomposed into an ETB trigger and an attacks trigger,
//!   both reanimating a land. ("tapped" rider is a fidelity partial —
//!   Reanimate has no tapped option.)
//! * "{G}, Discard a land card: You gain 3 life."
//! * "{R}, Discard a land card: Draw a card."
//! * "{B}, Discard a land card: Soul of Windgrace gains indestructible
//!   until end of turn. Tap it."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Soul of Windgrace");
    let cat = reg.interner_mut().intern("Cat");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Land filter for the "discard a land card" costs.
    let land_in_hand = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let land_in_hand2 = ObjectFilter::permanent().with_types(TypeLine::LAND.into());
    let land_in_hand3 = ObjectFilter::permanent().with_types(TypeLine::LAND.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: reanimate_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: reanimate_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}, Discard a land card: You gain 3 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    discard_other: Some(land_in_hand),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{R}, Discard a land card: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    discard_other: Some(land_in_hand2),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, Discard a land card: Soul of Windgrace gains indestructible until end of turn. Tap it."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    discard_other: Some(land_in_hand3),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: indestructible_and_tap,
            }),
    )
}

fn reanimate_land(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Reanimate {
        player: trig.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        from_zone: Zone::Graveyard(trig.controller),
    }]
}

fn gain_three(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: 3,
    }]
}

fn draw_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}

fn indestructible_and_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GrantKeyword {
            target: ctx.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
        Effect::Tap { target: ctx.source },
    ]
}
