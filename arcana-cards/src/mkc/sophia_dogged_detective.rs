//! Sophia, Dogged Detective — `{1}{G}{W}{U}` 3/4 Legendary Human Detective.
//! When Sophia enters, create Tiny, a legendary 2/2 green Dog Detective
//! token with trample.
//! `{1}, Sacrifice an artifact token: Put a +1/+1 counter on each Dog
//! you control.`
//! Whenever a Dog you control deals combat damage to a player, create a
//! Food token, then investigate.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sophia, Dogged Detective");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);
    // Pre-intern token strings so resolvers can recover them via lookup.
    let _tiny = reg.interner_mut().intern("Tiny");
    let _dog = reg.interner_mut().intern("Dog");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Investigate / Food are mechanic keywords, not KeywordAbility
        // variants; their effects are wired in the abilities below.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_tiny,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Sacrifice an artifact token: Put a +1/+1 counter on each Dog you control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice_other: Some(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::ARTIFACT.into())
                            .controlled_by(ControllerConstraint::You)
                            .tokens_only(),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_each_dog,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                // GAP: source_filter restricts to creatures you control but
                // not the "Dog" subtype — DamageDealt fires for any of your
                // creatures dealing combat damage to a player.
                intervening_if: None,
                effect: food_then_investigate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_tiny(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let tiny = reg.interner().lookup("Tiny").unwrap_or_default();
    let dog = reg.interner().lookup("Dog").unwrap_or_default();
    let detective = reg.interner().lookup("Detective").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(detective);
    // GAP: TokenDefinition carries no supertypes — Tiny's "legendary" is
    // not expressible on the token.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: tiny,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Trample],
            abilities: vec![],
        },
    }]
}

fn counter_each_dog(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Dog").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}

fn food_then_investigate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Food,
            count: 1,
        },
        Effect::CreateCommodityToken {
            controller: trig.controller,
            kind: CommodityToken::Clue,
            count: 1,
        },
    ]
}
