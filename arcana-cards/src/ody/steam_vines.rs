//! Steam Vines — `{1}{R}{R}` enchantment — Aura.
//! "Enchant land. When enchanted land becomes tapped, destroy it and
//! this Aura deals 1 damage to that land's controller. That player
//! attaches this Aura to a land of their choice."
//!
//! Enchant-land Aura with a host becomes-tapped trigger
//! (`AttachedCreatureDoes { SelfBecomesTapped }`). The host land is
//! reached via `source.attached_to` (mirroring Enslave / Mire Blight),
//! so we destroy it and deal 1 damage to that land's controller. GAP:
//! the final "that player attaches this Aura to a land of their choice"
//! re-attach has no demonstrated controlled-re-attach primitive.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Steam Vines");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesTapped),
                },
                intervening_if: None,
                effect: on_host_tapped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_host_tapped(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "that player attaches this Aura to a land of their choice" —
    // no controlled-re-attach primitive. Destroy + 1 damage are faithful.
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![
        Effect::DestroyPermanent { target: host },
        Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(controller),
            amount: 1,
        },
    ]
}
