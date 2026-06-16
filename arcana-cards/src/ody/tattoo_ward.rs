//! Tattoo Ward — `{2}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1 and has protection from
//!  enchantments. Sacrifice this Aura: Destroy target enchantment."
//!
//! Buff Aura. The +1/+1 is an ETB-installed `attached_pt`. "Protection from
//! enchantments" is protection from a card TYPE (no ProtectionQuality variant) —
//! GAP. The "Sacrifice this Aura: destroy target enchantment" self-activated
//! ability runs on the AURA itself (not the host) and is outside the aura
//! attached-grant surface — GAP.

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
    let name = reg.interner_mut().intern("Tattoo Ward");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: protection from enchantments (a card TYPE) — no ProtectionQuality variant.
    // GAP: "Sacrifice this Aura: Destroy target enchantment" — self-activated ability
    // on the Aura, outside the aura attached-grant surface.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
