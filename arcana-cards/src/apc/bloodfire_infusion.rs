//! Bloodfire Infusion — `{2}{R}` enchantment — Aura.
//! "Enchant creature you control. {R}, Sacrifice enchanted creature: This
//!  Aura deals damage equal to the sacrificed creature's power to each
//!  creature."
//!
//! Host-activated grant: the enchanted creature gains "{R}, Sacrifice this
//! creature: …" via an `attached_activated` whose cost includes
//! `sacrifice: true` (the ability's source is the host, so sacrificing it
//! sacrifices the enchanted creature). The damage payoff — "damage equal
//! to the sacrificed creature's power to EACH creature" — has no
//! expressible dynamic-power damage-to-each primitive here, so the effect
//! is GAP'd (the cost is faithful).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodfire Infusion");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: "creature you control" approximated by caster's Creature choice.
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let ability = ActivatedAbilityDef {
        text: "{R}, Sacrifice this creature: This Aura deals damage equal to \
               this creature's power to each creature."
            .into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{R}").expect("valid cost"),
            tap: false,
            sacrifice: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: false,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect: deal_power_to_each,
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            trig.source,
            ability,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn deal_power_to_each(
    _state: &GameState,
    _ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "deals damage equal to the sacrificed creature's power to each
    // creature" — no dynamic-power damage-to-each primitive expressible here.
    Vec::new()
}
