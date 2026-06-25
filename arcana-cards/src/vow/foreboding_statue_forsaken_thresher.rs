//! Foreboding Statue // Forsaken Thresher — `{3}` colorless Artifact Creature
//! — Construct 1/2. Transforms to Artifact Creature — Construct (back face).
//!
//! Front face:
//! {T}: Add one mana of any color. Put an omen counter on this creature.
//! "any color" is modeled as five front-face {T} mana abilities, one per
//!   WUBRG color (each also puts an omen counter); the shared tap cost means
//!   only one fires.
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
            // Five mana abilities, one per color (each also adds an omen counter).
            .with_activated_ability(fs_mana_ability(
                "{T}: Add {W}. Put an omen counter on this creature.",
                fs_white,
            ))
            .with_activated_ability(fs_mana_ability(
                "{T}: Add {U}. Put an omen counter on this creature.",
                fs_blue,
            ))
            .with_activated_ability(fs_mana_ability(
                "{T}: Add {B}. Put an omen counter on this creature.",
                fs_black,
            ))
            .with_activated_ability(fs_mana_ability(
                "{T}: Add {R}. Put an omen counter on this creature.",
                fs_red,
            ))
            .with_activated_ability(fs_mana_ability(
                "{T}: Add {G}. Put an omen counter on this creature.",
                fs_green,
            ))
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

fn fs_mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
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
        effect,
    }
}

fn fs_mana_and_omen(
    ctx: &ActivationContext,
    reg: &CardRegistry,
    color: ManaColor,
) -> Vec<Effect> {
    let omen_kind = match reg.interner().lookup("omen") {
        Some(s) => CounterKind::Named(s),
        // Fallback: use Charge as a stand-in (should not happen after register)
        None => CounterKind::Charge,
    };
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(color, ctx.source)],
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: omen_kind,
            count: 1,
        },
    ]
}

fn fs_white(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    fs_mana_and_omen(ctx, reg, ManaColor::White)
}
fn fs_blue(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    fs_mana_and_omen(ctx, reg, ManaColor::Blue)
}
fn fs_black(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    fs_mana_and_omen(ctx, reg, ManaColor::Black)
}
fn fs_red(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    fs_mana_and_omen(ctx, reg, ManaColor::Red)
}
fn fs_green(_s: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    fs_mana_and_omen(ctx, reg, ManaColor::Green)
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
