//! Seizures — `{1}{B}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature becomes tapped, this Aura
//!  deals 3 damage to that creature's controller unless that player pays {3}."
//!
//! Host-trigger Aura wrapping `SelfBecomesTapped`. The payoff deals 3 damage
//! to the enchanted creature's controller. The "unless that player pays {3}"
//! escape clause has no representable cost gate on a triggered effect, so the
//! damage is dealt unconditionally.
//! GAP: "unless that player pays {3}" — no unless-pay gate on triggers.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Seizures");
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
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesTapped),
                },
                intervening_if: None,
                effect: damage_host_controller,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damage_host_controller(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "unless that player pays {3}" — no unless-pay gate on a triggered
    // effect; the 3 damage is dealt unconditionally.
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to)
    else {
        return Vec::new();
    };
    let Some(controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(controller),
        amount: 3,
    }]
}
