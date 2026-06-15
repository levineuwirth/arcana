//! Aqueous Form — `{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature can't be blocked.
//!  Whenever enchanted creature attacks, scry 1."
//!
//! Host-trigger Aura. The "can't be blocked" evasion grant has no
//! `attached_*` builder in the demonstrated API (only cant_attack /
//! cant_block exist) — GAP. The scry-on-attack is a host trigger
//! (`AttachedCreatureDoes { SelfAttacks }`) that scries 1 for the Aura's
//! controller. The ETB install fn returns nothing (the evasion gap).

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aqueous Form");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
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
                // GAP: "enchanted creature can't be blocked" — no
                // attached can't-be-blocked / evasion builder available.
                effect: |_, _, _| Vec::new(),
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
                effect: scry_on_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn scry_on_attack(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(you) = state.objects.get(trig.source).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::Scry { player: you, count: 1 }]
}
