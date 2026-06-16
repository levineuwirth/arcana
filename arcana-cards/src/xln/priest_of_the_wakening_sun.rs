//! Priest of the Wakening Sun — `{W}` 1/1 Human Cleric.
//! At the beginning of your upkeep, you may reveal a Dinosaur card from
//! your hand. If you do, you gain 2 life.
//! {3}{W}{W}, Sacrifice this creature: Search your library for a Dinosaur
//! card, reveal it, put it into your hand, then shuffle.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Priest of the Wakening Sun");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let _dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_reveal_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}{W}, Sacrifice this creature: Search your library for a Dinosaur card, reveal it, put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}{W}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_dinosaur,
            }),
    )
}

fn upkeep_reveal_gain(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may reveal a Dinosaur card from your hand. If you do, gain 2
    // life." — there is no reveal-from-hand effect, and the life gain is gated
    // on that optional reveal, which cannot be expressed; emit nothing rather
    // than gaining life unconditionally.
    Vec::new()
}

fn tutor_dinosaur(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Dinosaur");
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter,
        reveal: true,
    }]
}
