//! Jerren, Corrupted Bishop // Ormendahl, the Corrupter — `{2}{B}` Legendary
//! Creature — Human Cleric 2/3.
//! Whenever Jerren enters or another nontoken Human you control dies, you lose
//! 1 life and create a 1/1 white Human creature token.
//! `{2}`: Target Human you control gains lifelink until end of turn.
//! At the beginning of your end step, if you have exactly 13 life, you may pay
//! `{4}{B}{B}`. If you do, transform Jerren.
//!
//! Back: Ormendahl, the Corrupter — Legendary Creature — Demon with flying,
//! trample, lifelink. "Sacrifice another creature: Draw a card."
//!
//! Transform DFC (CR 712). Triggers/abilities face-gated to their owning face.
//!
//! GAP: the dies-trigger filter ("another nontoken Human you control") cannot
//! exclude Jerren itself from the match.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, PlayerId, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jerren, Corrupted Bishop");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Ormendahl, the Corrupter — Legendary Demon, Flying/Trample/Lifelink.
    let back_name = reg.interner_mut().intern("Ormendahl, the Corrupter");
    let demon = reg.interner_mut().intern("Demon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(demon);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![
                KeywordAbility::Flying,
                KeywordAbility::Trample,
                KeywordAbility::Lifelink,
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    // Dies-trigger filter: another nontoken Human you control.
    let human_dies_filter = script::subtype_filter(reg, "Human")
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    // Activated target: a Human you control.
    let human_target_filter = script::subtype_filter(reg, "Human")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: Whenever Jerren enters, lose 1 life + make a 1/1 Human.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: lose_make_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Front: Whenever another nontoken Human you control dies, same.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: human_dies_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: lose_make_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Front: At your end step, if exactly 13 life, may pay {4}{B}{B}: transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_exactly_thirteen_life),
                effect: maybe_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 0)
            // Front: {2}: Target Human you control gains lifelink until EOT.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Target Human you control gains lifelink until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(human_target_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: grant_lifelink,
            })
            // Back: Sacrifice another creature: Draw a card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another creature: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: draw_one,
            }),
    )
}

fn if_exactly_thirteen_life(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::life_at_least(s, you, 13) && conditions::life_at_most(s, you, 13)
}

fn lose_make_token(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let human = reg
        .interner()
        .lookup("Human")
        .expect("Human interned during register");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::LoseLife { player: trig.controller, amount: 1 },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}

fn maybe_transform(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}

fn grant_lifelink(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Lifelink,
        duration: Duration::EndOfTurn,
    }]
}

fn draw_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}
