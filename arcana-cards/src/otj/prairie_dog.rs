//! Prairie Dog — `{1}{W}` 2/2 Creature — Squirrel with Lifelink.
//!
//! Oracle:
//! * Lifelink (base keyword).
//! * At the beginning of your end step, if you haven't cast a spell from your
//!   hand this turn, put a +1/+1 counter on this creature. (Wired as an
//!   end-step trigger gated by an intervening-if on spells-cast-this-turn == 0.
//!   FIDELITY GAP: the "from your hand" qualifier is approximated by the
//!   total spells-cast count.)
//! * {4}{W}: Until end of turn, if you would put one or more +1/+1 counters on
//!   a creature you control, put that many plus one instead. (A one-shot
//!   counter-doubling replacement-effect installer; no demonstrated Effect
//!   primitive installs such a replacement, so the effect body is GAP'd. The
//!   activated ability shell with its {4}{W} cost is still wired.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prairie Dog");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_no_spell_cast),
                effect: add_counter_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}: Until end of turn, if you would put one or more +1/+1 counters on a creature you control, put that many plus one +1/+1 counters on it instead.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_doubler,
            }),
    )
}

fn if_no_spell_cast(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    script::spells_cast_this_turn(s, &ObjectFilter::default(), you) == 0
}

fn add_counter_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn counter_doubler(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Until end of turn, if you would put one or more +1/+1 counters on a
    // creature you control, put that many plus one instead." — a one-shot
    // counter-amount replacement-effect installer; no demonstrated Effect
    // primitive installs such a replacement.
    Vec::new()
}
