//! Skeleton Crew — `{3}{B}` 3/3 black Skeleton Pirate.
//!
//! Oracle:
//! * "Each other creature you control that's a Skeleton or Pirate gets
//!   +1/+1." — a pure continuous STATIC anthem. The demonstrated API has
//!   no static-continuous primitive on a creature CardDefinition, so this
//!   line is GAP'd (see register()).
//! * "Whenever one or more creature cards leave your graveyard, create a
//!   2/2 black Skeleton Pirate creature token." — TriggeredAbilityDef:
//!   ZoneChange from your graveyard, creature filter controlled by you.
//! * "{5}{B}: Return this card from your graveyard to the battlefield
//!   tapped." — ActivatedAbilityDef, activation_zone Graveyard, self-return.
//!   The "tapped" rider has no field on ReturnFromGraveyardToBattlefield;
//!   the return is emitted (fidelity gap: enters untapped).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skeleton Crew");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(skeleton);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static — "Each other creature you control that's a Skeleton or
    // Pirate gets +1/+1." No static-continuous anthem primitive is exposed
    // for a creature CardDefinition in the demonstrated API.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Graveyard(0)),
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: create_skeleton_pirate_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{B}: Return this card from your graveyard to the battlefield tapped.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self_from_graveyard,
            }),
    )
}

/// "Whenever one or more creature cards leave your graveyard, create a
/// 2/2 black Skeleton Pirate creature token."
fn create_skeleton_pirate_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg
        .interner()
        .lookup("Skeleton")
        .unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(skeleton) = reg.interner().lookup("Skeleton") {
        subtypes.0.insert(skeleton);
    }
    if let Some(pirate) = reg.interner().lookup("Pirate") {
        subtypes.0.insert(pirate);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// "{5}{B}: Return this card from your graveyard to the battlefield tapped."
/// Fidelity gap: the "tapped" rider has no field on
/// ReturnFromGraveyardToBattlefield, so the card enters untapped.
fn return_self_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
