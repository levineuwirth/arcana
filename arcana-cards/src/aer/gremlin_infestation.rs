//! Gremlin Infestation — `{3}{R}` enchantment — Aura.
//! "Enchant artifact. At the beginning of your end step, this Aura deals
//!  2 damage to enchanted artifact's controller. When enchanted artifact
//!  is put into a graveyard, create a 2/2 red Gremlin creature token."
//!
//! Two abilities besides the fixed ETB boilerplate:
//!  - a `StepBegins(End, You)` trigger that deals 2 damage to the
//!    controller of the enchanted artifact (read via `source.attached_to`);
//!  - an `AttachedCreatureDoes(SelfDies)` trigger that fires when the
//!    enchanted artifact is put into a graveyard, creating a 2/2 red
//!    Gremlin token.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::TokenDefinition;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gremlin Infestation");
    let aura = reg.interner_mut().intern("Aura");
    // Interned here so the token-creating effect fn can `lookup` it.
    let _gremlin = reg.interner_mut().intern("Gremlin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                arcana_core::targets::ObjectFilter::permanent()
                    .with_types(TypeLine::ARTIFACT.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: make_gremlin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(src) = state.objects.get(trig.source) else {
        return Vec::new();
    };
    let Some(host) = src.attached_to else {
        return Vec::new();
    };
    let Some(host_obj) = state.objects.get(host) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(host_obj.controller),
        amount: 2,
    }]
}

fn make_gremlin(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let gremlin = reg.interner().lookup("Gremlin").expect("Gremlin interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gremlin);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: gremlin,
            colors: ColorSet::red(),
            types: TypeLine(TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
