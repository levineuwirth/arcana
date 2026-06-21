//! Departed Deckhand — `{1}{U}` 2/2 Spirit Pirate.
//! "When this creature becomes the target of a spell, sacrifice it.
//!  This creature can't be blocked except by Spirits.
//!  {3}{U}: Another target creature you control can't be blocked this
//!  turn except by Spirits."
//!
//! The becomes-targeted trigger is wired (self-removal). The two
//! "can't be blocked except by Spirits" lines are GAPs — the usable
//! CantBeBlocked Effect is unconditional, so emitting it would make the
//! creature unblockable by EVERYTHING (materially wrong).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Departed Deckhand");
    let spirit = reg.interner_mut().intern("Spirit");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP (static evasion): "This creature can't be blocked except by Spirits"
    // — the usable CantBeBlocked Effect is unconditional, so the
    // except-by-Spirits qualifier can't be expressed.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: sacrifice_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Another target creature you control can't be blocked this turn \
                       except by Spirits."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").expect("valid cost"),
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
                effect: cant_be_blocked_except_spirits,
            }),
    )
}

fn sacrifice_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): "sacrifice it" — no immediate sacrifice-this-object
    // Effect; routed through DestroyPermanent on the source (id-precise, but
    // differs from a true sacrifice for indestructible/regenerating cases).
    vec![Effect::DestroyPermanent { target: trig.source }]
}

fn cant_be_blocked_except_spirits(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can't be blocked this turn except by Spirits" — the usable
    // CantBeBlocked Effect is unconditional (no by-subtype exception), so
    // emitting it would make the target unblockable by everything. Targeting
    // is wired; the conditional-evasion payload is GAP'd.
    Vec::new()
}
