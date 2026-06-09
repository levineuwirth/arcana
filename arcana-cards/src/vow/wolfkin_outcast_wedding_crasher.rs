//! Wolfkin Outcast // Wedding Crasher — `{5}{G}` Human Werewolf 5/4 (front).
//! This spell costs {2} less to cast if you control a Wolf or Werewolf.
//! Daybound.
//! Back face (Wedding Crasher): Werewolf.
//! Whenever this creature or another Wolf or Werewolf you control dies, draw a card.
//! Nightbound.
//!
//! GAP: "This spell costs {2} less to cast if you control a Wolf or Werewolf" —
//! cost-reduction as a cast-time check is not expressible (no cost-reduction engine).
//! Daybound: the front (day) face transforms to the night face when it becomes
//! night, modeled by gating the upkeep transform trigger's `intervening_if` on
//! `conditions::it_is_night` (the front→back transform no longer fires during day).
//! GAP: Nightbound back-transform (back→front when it becomes day) not modeled.
//! GAP: back-face-only triggered ability (Wolf/Werewolf dies → draw a card) not
//! auto-installed on transform.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolfkin Outcast");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Wedding Crasher");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Daybound: front (day) face transforms to night face when it is night.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_it_is_night),
                effect: daybound_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: Nightbound back-transform and back-face triggered ability not modeled.
        // GAP: back-face-only triggered ability (Wolf/Werewolf dies → draw a card)
        //      not auto-installed on transform.
    )
}

fn iif_it_is_night(state: &GameState, _source: ObjectId, _you: PlayerId) -> bool {
    conditions::it_is_night(state)
}

fn daybound_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Front (day) → back (night) transform; gated by intervening_if (it is night).
    vec![Effect::Transform { target: trig.source }]
}
