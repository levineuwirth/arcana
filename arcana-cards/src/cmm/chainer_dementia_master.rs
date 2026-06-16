//! Chainer, Dementia Master — `{3}{B}{B}` 3/3 Legendary Human Minion.
//! "All Nightmares get +1/+1.
//!  {B}{B}{B}, Pay 3 life: Put target creature card from a graveyard onto
//!  the battlefield under your control. That creature is black and is a
//!  Nightmare in addition to its other creature types.
//!  When Chainer leaves the battlefield, exile all Nightmares."
//!
//! GAP (static): "All Nightmares get +1/+1" is a continuous anthem — not a
//! triggered/activated ability.
//! GAP (riders on the reanimation): the reanimated creature returns under its
//! owner's control (no documented targeted reanimate-under-your-control
//! variant), and the permanent "becomes black / is a Nightmare" riders have
//! no documented permanent SetColor / add-subtype effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Chainer, Dementia Master");
    let human = reg.interner_mut().intern("Human");
    let minion = reg.interner_mut().intern("Minion");
    let _nightmare = reg.interner_mut().intern("Nightmare");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(minion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{B}{B}, Pay 3 life: Put target creature card from a graveyard onto the battlefield under your control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{B}{B}").expect("valid cost"),
                    life: 3,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reanimate_creature,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfLeavesBattlefield,
                intervening_if: None,
                effect: exile_all_nightmares,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn reanimate_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}

fn exile_all_nightmares(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Nightmare").controlled_by(ControllerConstraint::Any),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent { target: NULL_OBJECT_ID }),
    }]
}
