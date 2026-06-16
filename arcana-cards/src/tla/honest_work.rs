//! Honest Work — `{U}` enchantment — Aura.
//! "Enchant creature an opponent controls. When this Aura enters, tap
//!  enchanted creature and remove all counters from it. Enchanted creature
//!  loses all abilities and is a Citizen with base power and toughness 1/1
//!  and '{T}: Add {C}' named Humble Merchant."
//!
//! Best-effort: ETB installs base power and toughness 1/1 via
//! `attached_set_pt`. The ETB tap + remove-all-counters action, "loses all
//! abilities", the subtype/name replacement to a Citizen named Humble
//! Merchant, and the granted "{T}: Add {C}" mana ability are not expressible
//! (subtype symbols cannot be interned inside the effect fn; no mana-ability
//! attached-grant primitive shown). Enchant target widened to any creature
//! (caster targets the opponent's creature).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Honest Work");
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
            // NOTE: controller wording approximated by caster's choice
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
    // GAP: ETB tap-and-remove-all-counters, "loses all abilities", rename/subtype
    // to Citizen "Humble Merchant", and the granted "{T}: Add {C}" mana ability.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_set_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
