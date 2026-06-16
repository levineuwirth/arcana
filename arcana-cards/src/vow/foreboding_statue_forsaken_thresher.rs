//! Foreboding Statue // Forsaken Thresher — `{3}` colorless Artifact Creature
//! — Construct 1/2. Transforms to Artifact Creature — Construct (back face).
//!
//! Front face:
//! {T}: Add one mana of any color. Put an omen counter on this creature.
//! GAP: "add one mana of any color" — any-color mana choice not expressible
//!   with a fixed ManaColor; modeled as colorless mana only.
//!
//! At the beginning of your end step, if there are three or more omen counters
//! on this creature, untap it, then transform it.
//! Modeled as end-step trigger with intervening-if guard approximation.
//! GAP: the exact "if 3+ omen counters on self" intervening_if condition is not
//!   expressible (no counter-count predicate in InterveningIf); fires
//!   unconditionally at end step and attempts transform. The engine's
//!   AddCounters effect does track Named counter "omen" faithfully.
//!
//! Back face — Forsaken Thresher:
//! At the beginning of your first main phase, add one mana of any color.
//! GAP: back-face triggered ability not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::targets::ControllerConstraint;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Foreboding Statue");
    let construct_sub = reg.interner_mut().intern("Construct");
    let omen_counter_name = reg.interner_mut().intern("omen");
    let _ = omen_counter_name; // interned for Named counter usage
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Forsaken Thresher");
    let back_construct_sub = reg.interner_mut().intern("Construct");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_construct_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {T}: Add one mana of any color. Put an omen counter on this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color. Put an omen counter on this creature.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: tap_for_mana_and_omen,
            })
            // Front face: At the beginning of your end step, if 3+ omen counters, untap then transform.
            // GAP: "if three or more omen counters" intervening-if not expressible;
            // fires unconditionally at end step.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_check_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face triggered ability (beginning of your first main
            // phase, add one mana of any color) not auto-installed on transform.
    )
}

fn tap_for_mana_and_omen(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "one mana of any color" — modeling as colorless since any-color
    // choice is not expressible with a fixed ManaColor.
    let omen_name = reg.interner().lookup("omen");
    let omen_kind = if let Some(s) = omen_name {
        CounterKind::Named(s)
    } else {
        // Fallback: use Charge as a stand-in (should not happen after register)
        CounterKind::Charge
    };
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: omen_kind,
            count: 1,
        },
    ]
}

fn end_step_check_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only fire if there are 3+ omen counters on this creature.
    // Intervening-if counter count check not expressible; fires unconditionally.
    vec![
        Effect::Untap { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
