//! Knight of the Ebon Legion — `{B}` 1/2 Vampire Knight.
//! `{2}{B}: This creature gets +3/+3 and gains deathtouch until end of
//! turn.` "At the beginning of your end step, if a player lost 4 or more
//! life this turn, put a +1/+1 counter on this creature." The end-step
//! trigger places the counter; its intervening-if ("a player lost 4+ life
//! this turn") has no conditions:: helper, so it is GAP'd to `None` (fires
//! unconditionally).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of the Ebon Legion");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: This creature gets +3/+3 and gains deathtouch until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_deathtouch,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening-if "if a player lost 4 or more life this turn"
                // — no conditions:: helper for per-turn life loss; fires
                // unconditionally.
                intervening_if: None,
                effect: add_counter_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_and_deathtouch(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Deathtouch],
    }]
}

fn add_counter_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
