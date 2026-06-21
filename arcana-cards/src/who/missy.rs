//! Missy — `{3}{U}{B}{R}` 4/5 legendary U/B/R Time Lord Rogue.
//!
//! * Whenever another nonartifact creature dies, return it to the
//!   battlefield under your control face down and tapped. It's a 2/2
//!   Cyberman artifact creature.
//! * At the beginning of your end step, each opponent faces a villainous
//!   choice — Each artifact creature you control deals 1 damage to that
//!   opponent, or you draw a card and chaos ensues.
//!
//! GAPs:
//! - Ability 1: the reanimation is non-faithful — it returns under YOUR
//!   control face down as a 2/2 Cyberman artifact creature (control change
//!   + face-down + base-PT set + type/subtype rewrite on a graveyard
//!   return), which is not composable from the usable effect catalog;
//!   GAP'd entirely.
//! - Ability 2: "villainous choice" (a player-facing branch) has no
//!   expressible primitive; GAP'd entirely.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Missy");
    let timelord = reg.interner_mut().intern("Time Lord");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(timelord);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let _cyberman = reg.interner_mut().intern("Cyberman");

    reg.register(
        CardDefinition::new(name, chars)
            // Whenever another nonartifact creature dies, … (GAP'd effect).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .without_types(TypeLine::ARTIFACT.into()),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: reanimate_as_cyberman_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // At the beginning of your end step, villainous choice (GAP'd).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: villainous_choice_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn reanimate_as_cyberman_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: return another nonartifact creature under YOUR control face
    // down as a 2/2 Cyberman artifact creature — the control change +
    // face-down base-PT + type/subtype rewrite on a graveyard return is
    // not composable from the usable effect catalog.
    Vec::new()
}

fn villainous_choice_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "villainous choice" player-facing branch has no expressible
    // primitive.
    Vec::new()
}
