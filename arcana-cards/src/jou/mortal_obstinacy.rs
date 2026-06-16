//! Mortal Obstinacy — `{W}` enchantment — Aura.
//! "Enchant creature you control. Enchanted creature gets +1/+1.
//!  Whenever enchanted creature deals combat damage to a player, you may
//!  sacrifice this Aura. If you do, destroy target enchantment."
//!
//! The +1/+1 is an ETB-installed `attached_pt`. The host combat-damage
//! payoff is keyed on "deals combat damage to a player" — not one of the
//! wrappable `Self*` conditions — and also bundles a may-sacrifice plus a
//! targeted destroy, so it cannot be expressed.
//! GAP: host "deals combat damage to a player → sacrifice, destroy target
//! enchantment" — not a Self* trigger condition.

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
    let name = reg.interner_mut().intern("Mortal Obstinacy");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        // NOTE: controller wording ("you control") approximated by caster's
        // choice of target.
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
    // GAP: host "Whenever enchanted creature deals combat damage to a player,
    // you may sacrifice this Aura; if you do, destroy target enchantment" —
    // combat-damage-to-a-player is not a wrappable Self* condition. Only the
    // +1/+1 buff is installed.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
