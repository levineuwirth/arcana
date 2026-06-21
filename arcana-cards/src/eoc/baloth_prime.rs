//! Baloth Prime — `{3}{G}` 10/10 green Beast.
//! Enters tapped with six stun counters.
//! Whenever you sacrifice a land, create a tapped 4/4 green Beast token
//! and untap this creature.
//! `{4}, Sacrifice a land`: You gain 2 life.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ControllerConstraint};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Baloth Prime");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "Enters tapped with six stun counters." The stun counters are added
            // on ETB. GAP: "enters tapped" — no enters-tapped hook in this API.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_stun_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever you sacrifice a land, create a tapped 4/4 Beast and untap."
            // GAP: the created token's "tapped" status is not expressible via a
            // plain CreateToken; the token is created untapped.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_sac_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "{4}, Sacrifice a land: You gain 2 life."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}, Sacrifice a land: You gain 2 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    sacrifice_other: Some(
                        ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_two_life,
            }),
    )
}

fn etb_stun_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Stun,
        count: 6,
    }]
}

fn on_sac_land(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: beast,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(4)),
                toughness: Some(PtValue::Fixed(4)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::Untap { target: trig.source },
    ]
}

fn gain_two_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 2 }]
}
