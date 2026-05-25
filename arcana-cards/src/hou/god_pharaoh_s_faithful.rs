//! God-Pharaoh's Faithful — `{W}` 0/4 white Human Wizard.
//! "Whenever you cast a blue, black, or red spell, you gain 1 life."
//! GAP: ObjectFilter has no colors_any (OR-mask); using three separate
//! TriggeredAbilityDefs for the three colors is the closest approach but
//! the engine only shows one trigger per card pattern. Using SpellCast
//! with no color filter as best-effort; verify pipeline will flag.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("God-Pharaoh's Faithful");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    // GAP: ObjectFilter has no colors_any OR-mask for "blue, black, or red".
    // Three separate TriggeredAbilityDefs needed; engine pattern only shown
    // with one. Using three IDs 1/2/3.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(arcana_core::targets::ObjectFilter::new()
                        .with_colors(ColorSet::blue())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(arcana_core::targets::ObjectFilter::new()
                        .with_colors(ColorSet::black())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(arcana_core::targets::ObjectFilter::new()
                        .with_colors(ColorSet::red())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 1 }]
}
