//! Gurzigost — `{3}{G}{G}` 6/8 Beast.
//! "At the beginning of your upkeep, sacrifice this creature unless you put two
//! cards from your graveyard on the bottom of your library." (graveyard-bottom
//! alternative cost not expressible — GAP)
//! "{G}{G}, Discard a card: You may have this creature assign its combat damage
//! this turn as though it weren't blocked." (no assign-as-unblocked Effect — GAP)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
// ObjectFilter used for the discard_other cost filter below.
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gurzigost");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_sac,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}{G}, Discard a card: You may have this creature assign its combat damage this turn as though it weren't blocked.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}{G}").expect("valid cost"),
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: assign_as_unblocked,
            }),
    )
}

fn upkeep_sac(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "sacrifice this unless you put two cards from your graveyard on the
    // bottom of your library" — the graveyard-to-bottom alternative cost is not
    // an OptionalPaymentKind (only Mana/Life), and putting chosen graveyard
    // cards on the bottom is not an available effect.
    Vec::new()
}

fn assign_as_unblocked(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "may have this creature assign its combat damage as though it weren't
    // blocked" — no assign-damage-as-unblocked Effect variant.
    Vec::new()
}
