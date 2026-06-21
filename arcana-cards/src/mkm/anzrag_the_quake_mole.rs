//! Anzrag, the Quake-Mole — `{2}{R}{G}` 8/4 legendary Mole God.
//! "Whenever Anzrag becomes blocked, untap each creature you control.
//! After this phase, there is an additional combat phase.
//! {3}{R}{R}{G}{G}: Anzrag must be blocked each combat this turn if
//! able."
//!
//! Abilities:
//! 1. SelfBecomesBlocked → untap each creature you control, then add an
//!    additional combat phase.
//! 2. {3}{R}{R}{G}{G}: "must be blocked each combat this turn if able"
//!    — there is no must-be-blocked / lure Effect in the catalog; GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Anzrag, the Quake-Mole");
    let mole = reg.interner_mut().intern("Mole");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mole);
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: untap_and_extra_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{R}{R}{G}{G}: Anzrag must be blocked each combat this turn \
                       if able."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}{R}{G}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: must_be_blocked,
            }),
    )
}

fn untap_and_extra_combat(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::Untap {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        },
        Effect::AdditionalCombatPhase,
    ]
}

fn must_be_blocked(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Anzrag must be blocked each combat this turn if able" — the
    // engine has no must-be-blocked / lure Effect, so this can't be
    // expressed. (ForbidBlocking/Goad are the inverse and don't apply.)
    Vec::new()
}
