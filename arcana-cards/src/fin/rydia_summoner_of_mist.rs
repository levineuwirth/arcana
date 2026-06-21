//! Rydia, Summoner of Mist — `{R}{G}` 1/2 Legendary Human Shaman.
//! Landfall — Whenever a land you control enters, you may discard a card. If
//! you do, draw a card.
//! Summon — {X}, {T}: Return target Saga card with mana value X from your
//! graveyard to the battlefield with a finality counter on it. It gains haste
//! until end of turn. Activate only as a sorcery.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rydia, Summoner of Mist");
    let human = reg.interner_mut().intern("Human");
    let shaman = reg.interner_mut().intern("Shaman");
    let saga = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let saga_filter =
        ObjectFilter::new().with_subtype_sym(saga);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(arcana_core::targets::ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_rummage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}, {T}: Return target Saga card with mana value X from \
                       your graveyard to the battlefield with a finality counter \
                       on it. It gains haste until end of turn. Activate only as \
                       a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: saga_filter,
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: summon_saga,
            }),
    )
}

fn landfall_rummage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you may discard a card. If you do, draw a card." Modeled as discard then
    // draw (the may-optionality of the discard is a documented fidelity gap).
    vec![Effect::Sequence(vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ])]
}

fn summon_saga(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the X-equals-mana-value coupling of the target Saga to the {X} paid
    // cannot be expressed in the target filter; any Saga card in a graveyard is
    // a legal target here.
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let finality = reg
        .interner()
        .lookup("finality")
        .map(CounterKind::Named);
    let mut effects = vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }];
    if let Some(kind) = finality {
        effects.push(Effect::AddCounters {
            target: *id,
            kind,
            count: 1,
        });
    }
    effects.push(Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Haste,
        duration: Duration::EndOfTurn,
    });
    effects
}
