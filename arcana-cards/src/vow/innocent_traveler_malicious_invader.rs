//! Innocent Traveler // Malicious Invader
//!
//! Front face: `{2}{B}{B}` Creature — Human 1/3.
//! At the beginning of your upkeep, any opponent may sacrifice a creature of
//! their choice. If no one does, transform this creature.
//! (GAP: this is a cross-opponent aggregate — "if NO ONE does, transform".
//! Effect::OptionalPayment is a single-chooser pay/decline gate; mapping one per
//! opponent can't express "transform only if EVERY opponent declined" (each
//! per-opponent else_effect would fire the transform even when another opponent
//! sacrificed). The aggregation blocker is unrelated to the sacrifice payment
//! itself. Emitting an unconditional transform on upkeep as best approximation;
//! the opponent-sacrifice gate is a GAP.)
//!
//! Back face: Creature — Vampire with Flying.
//! This creature gets +2/+0 as long as an opponent controls a Human.
//! (GAP: conditional static +2/+0 while opponent controls a Human — no
//! static layer effect expressible; back-face-only static ability not
//! modeled.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Innocent Traveler");
    let human_sub = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Malicious Invader");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            // GAP: static +2/+0 while opponent controls a Human not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    // Front face trigger: at beginning of your upkeep, any opponent may
    // sacrifice a creature. If no one does, transform.
    // GAP: cross-opponent "if NO ONE does" aggregate can't be expressed by a
    // single-chooser OptionalPayment (see header). Approximating as an
    // unconditional transform trigger on upkeep — the opponent-sacrifice gate is
    // omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only static "+2/+0 as long as an opponent controls a
        // Human" not modeled.
    )
}

fn upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Should prompt each opponent to optionally sacrifice a creature; only
    // transforms if none do. The cross-opponent "if no one does" aggregate is
    // not expressible by single-chooser OptionalPayment gates — emitting
    // unconditional Transform as best approximation.
    vec![Effect::Transform { target: trig.source }]
}
