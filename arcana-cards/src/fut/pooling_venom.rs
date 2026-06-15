//! Pooling Venom — `{1}{B}` enchantment — Aura.
//! "Enchant land. Whenever enchanted land becomes tapped, its controller
//! loses 2 life. {3}{B}: Destroy enchanted land."
//!
//! The becomes-tapped payoff is a host trigger
//! (`AttachedCreatureDoes { SelfBecomesTapped }`) that drains the land's
//! controller for 2. The `{3}{B}: Destroy enchanted land` is the Aura's
//! own activated ability, destroying `source.attached_to`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pooling Venom");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesTapped),
                },
                intervening_if: None,
                effect: tapped_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}: Destroy enchanted land.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").expect("valid cost"),
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_enchanted,
            }),
    )
}

fn tapped_drain(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to)
    else {
        return Vec::new();
    };
    let Some(controller) = state.objects.get(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::LoseLife { player: controller, amount: 2 }]
}

fn destroy_enchanted(
    state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.objects.get(ctx.source).and_then(|o| o.attached_to)
    else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: host }]
}
