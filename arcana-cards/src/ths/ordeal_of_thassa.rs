//! Ordeal of Thassa — `{1}{U}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature attacks, put a +1/+1
//!  counter on it. Then if it has three or more +1/+1 counters on it,
//!  sacrifice this Aura. When you sacrifice this Aura, draw two cards."
//!
//! Host-trigger Aura (the Ordeal cycle, cf. Ordeal of Nylea). An
//! `AttachedCreatureDoes(SelfAttacks)` trigger puts a +1/+1 counter on the
//! enchanted (attacking) creature via `trig.attacking_creature()`. The
//! conditional self-sacrifice ("if it has 3+ counters, sacrifice this Aura")
//! and the "when you sacrifice this Aura, draw two cards" payoff are GAP'd —
//! the conditional self-sacrifice-then-draw chain has no expressible
//! primitive in the demonstrated API.

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
    let name = reg.interner_mut().intern("Ordeal of Thassa");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfAttacks),
                },
                intervening_if: None,
                effect: on_attack_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_noop(
    _state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
}

fn on_attack_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then if it has 3+ +1/+1 counters, sacrifice this Aura" and
    // "when you sacrifice this Aura, draw two cards" — conditional
    // self-sacrifice-then-draw chain not expressible. Models only the
    // counter placement.
    let Some(host) = trig.attacking_creature() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: host,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
