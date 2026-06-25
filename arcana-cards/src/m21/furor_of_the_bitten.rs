//! Furor of the Bitten — `{R}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +2/+2 and attacks each combat
//!  if able."
//!
//! The +2/+2 buff is an ETB-installed `attached_pt`. "Attacks each combat
//! if able" installs a must-attack requirement (CR 508.1a) on the enchanted
//! creature, read from `source.attached_to` at install time. (There is no
//! attached-creature must-attack builder that follows re-attachment, so the
//! requirement is pinned to the host the Aura is on when it enters; this Aura
//! does not move, so that is faithful.)

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
    let name = reg.interner_mut().intern("Furor of the Bitten");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut out = vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }];
    // "Enchanted creature ... attacks each combat if able." — install a
    // must-attack requirement on the host (read from `attached_to`).
    if let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to) {
        out.push(Effect::InstallContinuousEffect {
            effect: ContinuousEffect::must_attack(
                trig.source,
                host,
                Duration::WhileSourceOnBattlefield,
            ),
        });
    }
    out
}
