//! Cephalid Illusionist — `{1}{U}` 1/1 Octopus Wizard.
//! Whenever this creature becomes the target of a spell or ability, mill three
//! cards.
//! `{2}{U}, {T}: Prevent all combat damage that would be dealt to and dealt by
//! target creature you control this turn.`
//!
//! Trigger: `SelfBecomesTarget { caster: Any }` → the controller mills three.
//! Activated ability: the "to" half is `Effect::PreventDamage` on the chosen
//! creature (FIDELITY GAP: prevents ALL damage, not just combat, since
//! PreventDamage has no combat-only field); the "by" half (damage dealt BY the
//! creature) is GAP'd — PreventDamageFrom filters its source by ObjectFilter
//! and cannot pin a single chosen object id.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cephalid Illusionist");
    let octopus = reg.interner_mut().intern("Octopus");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: mill_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, {T}: Prevent all combat damage that would be dealt to and dealt by target creature you control this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_to_target,
            }),
    )
}

fn mill_three(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill {
        player: trig.controller,
        count: 3,
    }]
}

fn prevent_to_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "to" half — prevent damage dealt TO the chosen creature this turn.
    // FIDELITY GAP: prevents all damage, not just combat (no combat-only field).
    // GAP: "dealt by target creature" — the by-half is not expressible:
    // PreventDamageFrom filters its source by ObjectFilter, not a single id.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
