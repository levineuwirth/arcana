//! Artifact Possession — `{2}{B}` enchantment — Aura.
//! "Enchant artifact. Whenever enchanted artifact becomes tapped or a
//!  player activates an ability of enchanted artifact without {T} in its
//!  activation cost, this Aura deals 2 damage to that artifact's
//!  controller."
//!
//! The becomes-tapped half is a host `AttachedCreatureDoes(SelfBecomesTapped)`
//! trigger that deals 2 damage to the host's controller. The
//! "activates an ability without {T}" half is not a Self* condition this
//! trigger can wrap — GAP'd (only the tap half fires).

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
    let name = reg.interner_mut().intern("Artifact Possession");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
            ))
            // GAP: "or a player activates an ability of enchanted artifact
            // without {T} in its activation cost" — not a Self* condition.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
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
    let Some(src) = state.objects.get(trig.source) else {
        return Vec::new();
    };
    let Some(host) = src.attached_to else {
        return Vec::new();
    };
    let Some(host_obj) = state.objects.get(host) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(host_obj.controller),
        amount: 2,
    }]
}
