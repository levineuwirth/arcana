//! Sabotage Strategist — `{2}{U}{U}` 2/2 Vedalken Ranger with Flying and
//! Vigilance.
//! "Whenever one or more creatures attack you, those creatures get -1/-0 until
//! end of turn." — modeled as a per-attacker CreatureAttacks trigger over
//! opponent-controlled creatures (the "you" defending-player restriction and
//! the one-or-more batching are approximated; see GAP).
//! "Exhaust — {5}{U}{U}: Put three +1/+1 counters on this creature." — modeled
//! as a plain activated ability; the once-per-game Exhaust limit is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sabotage Strategist");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(ranger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "attack you" defending-player restriction + the
                //      one-or-more batch are approximated as a per-attacker
                //      trigger over opponent-controlled attacking creatures.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: weaken_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {5}{U}{U}: Put three +1/+1 counters on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{U}{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                // GAP: Exhaust "activate only once" lifetime limit not modeled.
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: three_counters,
            }),
    )
}

fn weaken_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: id,
        power: -1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn three_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 3,
    }]
}
