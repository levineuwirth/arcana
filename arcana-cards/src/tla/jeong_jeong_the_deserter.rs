//! Jeong Jeong, the Deserter — `{2}{R}` 2/3 Legendary Creature — Human Rebel
//! Ally.
//!
//! * Firebending 1 (Whenever this creature attacks, add {R}. This mana lasts
//!   until end of combat.) — Firebending is not in the usable keyword surface,
//!   but its rules text is a "Whenever ~ attacks, add {R}" triggered ability,
//!   so we wire it as a triggered ability with `Effect::AddMana`.
//! * Exhaust — {3}: Put a +1/+1 counter on Jeong Jeong. When you next cast a
//!   Lesson spell this turn, copy it and you may choose new targets for the
//!   copy. (Activate each exhaust ability only once.)
//!   We wire the {3} activated ability with `once_per_turn` (best-effort for
//!   "activate each exhaust ability only once") and add the +1/+1 counter; the
//!   "next Lesson spell you cast, copy it" delayed rider is GAP'd (no
//!   next-cast Lesson-copy primitive in this surface).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jeong Jeong, the Deserter");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Firebending 1 — "Whenever this creature attacks, add {R}."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: firebending_add_red,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Exhaust — {3}: Put a +1/+1 counter on Jeong Jeong. (next-Lesson
            // copy rider GAP'd)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {3}: Put a +1/+1 counter on Jeong Jeong. When you next cast a Lesson spell this turn, copy it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exhaust_counter,
            }),
    )
}

/// Firebending 1: the attacking creature's controller adds {R}.
fn firebending_add_red(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source)],
    }]
}

/// Exhaust: put a +1/+1 counter on Jeong Jeong.
/// GAP: "When you next cast a Lesson spell this turn, copy it and you may
/// choose new targets" — no next-cast Lesson-copy rider primitive here.
fn exhaust_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
