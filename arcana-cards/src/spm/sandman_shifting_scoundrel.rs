//! Sandman, Shifting Scoundrel — `{1}{G}{G}` */* Legendary Creature —
//! Sand Elemental Villain (green).
//!
//! * "Sandman's power and toughness are each equal to the number of
//!   lands you control." — CDA wired at Layer 7a via a SelfEntersBattlefield
//!   self_pt_from_match counting lands you control (symmetric). Bones `*/*`.
//! * "Sandman can't be blocked by creatures with power 2 or less." —
//!   GAP: a filtered "can't be blocked by [filter]" static; the only
//!   evasion primitive is unconditional CantBeBlocked.
//! * "{3}{G}{G}: Return this card and target land card from your
//!   graveyard to the battlefield tapped." — implemented as a
//!   graveyard-activated ability returning both cards. FIDELITY GAP:
//!   the returns enter untapped (no graveyard-return-tapped primitive).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sandman, Shifting Scoundrel");
    let sand = reg.interner_mut().intern("Sand");
    let elemental = reg.interner_mut().intern("Elemental");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sand);
    subtypes.0.insert(elemental);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // */* — CDA "P/T equal to lands you control" wired below.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // CDA: P/T each equal to the number of lands you control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "{3}{G}{G}: Return this card and target land card from your
            // graveyard to the battlefield tapped."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}{G}: Return this card and target land card from \
                       your graveyard to the battlefield tapped."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into()),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self_and_land,
            }),
    )
}

/// Layer 7a self-CDA: P/T each equal to the number of lands you control.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn return_self_and_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::ReturnFromGraveyardToBattlefield {
        target: ctx.source,
    }];
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
    }
    effects
}
