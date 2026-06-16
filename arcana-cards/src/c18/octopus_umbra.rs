//! Octopus Umbra — `{3}{U}{U}` enchantment — Aura.
//! "Enchant creature. Enchanted creature has base power and toughness 8/8 and
//!  has \"Whenever this creature attacks, you may tap target creature with
//!  power 8 or less.\" Umbra armor."
//!
//! The base-P/T set to 8/8 is an ETB-installed `attached_set_pt`. The granted
//! "whenever this creature attacks…" is a TRIGGERED ability granted to the
//! host (attached_activated only grants ACTIVATED abilities) — GAP. Umbra armor
//! (a destruction-replacement on the host that instead destroys this Aura) is
//! a replacement effect — GAP.

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
    let name = reg.interner_mut().intern("Octopus Umbra");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
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
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: granted "whenever this creature attacks, tap target" host TRIGGERED
    // ability; GAP: Umbra armor (destruction-replacement totem armor).
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_set_pt(trig.source, 8, 8, Duration::WhileSourceOnBattlefield),
    }]
}
