//! The Abyss — `{3}{B}` World Enchantment.
//! "At the beginning of each player's upkeep, destroy target
//! nonartifact creature that player controls of their choice. It can't
//! be regenerated."
//!
//! GAP: the World supertype is not in the demonstrated SupertypeSet
//! surface (LEGENDARY / BASIC only) — registered as a plain
//! enchantment; the World rule (CR 704.5m) is unmodeled anyway.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Abyss");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: drag_into_the_abyss,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "that player controls of their choice" — the target
            // should be constrained to the active player's creatures and
            // chosen by that player; neither is expressible, so this is
            // a plain nonartifact-creature target chosen by the
            // ability's controller.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().without_types(TypeLine::ARTIFACT.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

/// "…destroy target nonartifact creature … It can't be regenerated."
fn drag_into_the_abyss(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "It can't be regenerated" — DestroyPermanent has no
    // no-regeneration rider.
    vec![Effect::DestroyPermanent { target: *id }]
}
