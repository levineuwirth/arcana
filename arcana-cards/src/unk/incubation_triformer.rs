//! Incubation Triformer — `{2}{B}` 3/3 Creature — Phyrexian Cleric.
//!
//! * "When Incubation Triformer enters the battlefield, incubate 1." — wired
//!   via `Effect::Incubate`.
//! * "{3}: Choose target transformed Phyrexian token you control. Transform it
//!   into a third side that's a 3/3 Phyrexian Horror artifact creature." —
//!   partially wired: a {3} activated ability targeting a Phyrexian creature
//!   you control and calling `Effect::Transform`. FIDELITY GAP: the "third
//!   side" (a distinct 3/3 Phyrexian Horror artifact-creature face) is not
//!   modeled; this performs a plain transform.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Incubation Triformer");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_incubate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}: Choose target transformed Phyrexian token you control. Transform it into a third side that's a 3/3 Phyrexian Horror artifact creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        script::subtype_filter(reg, "Phyrexian")
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: transform_phyrexian,
            }),
    )
}

fn etb_incubate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Incubate {
        controller: trig.controller,
        n: 1,
    }]
}

fn transform_phyrexian(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // FIDELITY GAP: the bespoke "third side" 3/3 Phyrexian Horror artifact
    // creature face is not modeled; perform a plain transform.
    vec![Effect::Transform { target: *id }]
}
