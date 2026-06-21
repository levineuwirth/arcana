//! Satsuki, the Living Lore — `{G}{W}` 1/3 Legendary Human Druid.
//!
//! * `{T}: Put a lore counter on each Saga you control. Activate only as
//!   a sorcery.`
//! * When Satsuki dies, choose up to one —
//!     • Return target Saga or enchantment creature you control to its
//!       owner's hand.  (implemented as the trigger's effect; both Sagas
//!       and enchantment creatures are enchantments, so the target filter
//!       is "enchantment you control")
//!     • Return target Saga card from your graveyard to your hand.  (the
//!       modal alternative is GAP'd — triggered modal choice + a second
//!       target set is not a documented shape.)

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
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Satsuki, the Living Lore");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Put a lore counter on each Saga you control. Activate only as a sorcery.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: lore_on_each_saga,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_death_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ENCHANTMENT.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn lore_on_each_saga(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let saga_filter =
        script::subtype_filter(reg, "Saga").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &saga_filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::Lore,
            count: 1,
        }),
    }]
}

fn on_death_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose up to one" modal with a second mode (return a Saga card
    // from your graveyard) is not a documented triggered-ability shape; only
    // mode 1 (return a target enchantment you control to hand) is emitted.
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}
