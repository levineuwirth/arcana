//! Spirit Shackle — `{B}{B}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature becomes tapped, put a
//!  -0/-2 counter on it."
//!
//! Host-trigger Aura. The grant is purely a host trigger
//! (`AttachedCreatureDoes { SelfBecomesTapped }`) that places a counter on
//! the host creature. There is no `-0/-2` `CounterKind`, so a `Named`
//! counter records the trigger faithfully but the engine does NOT apply the
//! -0/-2 power/toughness modification from it — that semantics is GAPped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirit Shackle");
    let aura = reg.interner_mut().intern("Aura");
    let _label = reg.interner_mut().intern("-0/-2");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
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
                effect: put_minus_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn put_minus_counter(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    // GAP: no `-0/-2` CounterKind; a Named counter records the trigger but the
    // engine does not derive a -0/-2 P/T change from it.
    let label = reg.interner().lookup("-0/-2").unwrap_or_default();
    vec![Effect::AddCounters {
        target: host,
        kind: CounterKind::Named(label),
        count: 1,
    }]
}
