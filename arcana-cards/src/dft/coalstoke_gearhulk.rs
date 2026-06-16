//! Coalstoke Gearhulk — `{1}{B}{B}{R}{R}` 5/4 Artifact Creature —
//! Construct with Menace and Deathtouch.
//! "When this creature enters, put target creature card with mana value
//! 4 or less from a graveyard onto the battlefield under your control
//! with a finality counter on it. That creature gains menace,
//! deathtouch, and haste. At the beginning of your next end step, exile
//! that creature."
//!
//! The ETB targets a creature card (mv ≤ 4) in a graveyard, reanimates
//! it (under the controller's control), places a finality counter,
//! grants the three keywords for the turn, and schedules an end-step
//! exile. The finality counter has no dedicated CounterKind variant, so
//! a Named("finality") counter is used.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coalstoke Gearhulk");
    let construct = reg.interner_mut().intern("Construct");
    let _finality = reg.interner_mut().intern("finality");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}{R}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: reanimate_and_exile,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_cmc(4),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_and_exile(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    let finality = reg
        .interner()
        .lookup("finality")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::PlusOnePlusOne);
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: id },
        Effect::AddCounters {
            target: id,
            kind: finality,
            count: 1,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Menace,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
        Effect::DelayedAction {
            source: id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Exile,
        },
    ]
}
