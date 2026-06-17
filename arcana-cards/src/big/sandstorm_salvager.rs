//! Sandstorm Salvager — `{2}{G}` 1/1 Human Artificer.
//!
//! Oracle:
//! * "When this creature enters, create a 3/3 colorless Golem artifact
//!   creature token."
//! * "{2}, {T}: Put a +1/+1 counter on each creature token you control.
//!   They gain trample until end of turn."
//!
//! Both abilities are wired. The activated ability uses two `ForEach`
//! passes over the creature tokens you control (ForEach substitutes each
//! id into the inner effect's placeholder target).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sandstorm Salvager");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    // Pre-intern the Golem token subtype so the resolver can rebuild it.
    let _golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_golem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Put a +1/+1 counter on each creature token you control. They gain trample until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_token_creatures,
            }),
    )
}

fn etb_make_golem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let golem = reg.interner().lookup("Golem").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: golem,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn pump_token_creatures(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .tokens_only();
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![
        Effect::ForEach {
            targets: ids.clone(),
            effect: Box::new(Effect::AddCounters {
                target: NULL_OBJECT_ID,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
        },
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Trample,
                duration: Duration::EndOfTurn,
            }),
        },
    ]
}
