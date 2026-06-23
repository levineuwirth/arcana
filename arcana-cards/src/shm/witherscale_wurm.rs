//! Witherscale Wurm — `{4}{G}{G}` 9/9 green Wurm.
//!
//! * Whenever this creature blocks or becomes blocked by a creature,
//!   that creature gains wither until end of turn.
//! * Whenever this creature deals damage to an opponent, remove all
//!   -1/-1 counters from it (this creature).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Witherscale Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBlocksOrBecomesBlocked,
                intervening_if: None,
                effect: grant_wither_to_other,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::default(),
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: remove_all_minus_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grant_wither_to_other(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.other_combatant() else { return Vec::new(); };
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Wither,
        duration: Duration::EndOfTurn,
    }]
}

fn remove_all_minus_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "remove all -1/-1 counters from it" — it = this creature.
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(CounterKind::MinusOneMinusOne));
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: CounterKind::MinusOneMinusOne,
        count: n,
    }]
}
