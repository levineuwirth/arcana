//! Sea's Claim — `{U}` enchantment — Aura.
//! "Enchant land. Enchanted land is an Island."
//!
//! Type-grant Aura on a land. `with_enchant` targets a land; the ETB installs
//! an `attached_subtypes` adding the Island subtype (additive) following
//! `source.attached_to`. The Island symbol is resolved from the registry's
//! interner inside the effect fn.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
    let name = reg.interner_mut().intern("Sea's Claim");
    let aura = reg.interner_mut().intern("Aura");
    // intern Island now so the symbol exists for the effect-time lookup.
    let _island = reg.interner_mut().intern("Island");
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
            .with_enchant(TargetFilter::Permanent(
                ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
            ))
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let mut grant = SubtypeSet::default();
    if let Some(island) = reg.interner().lookup("Island") {
        grant.0.insert(island);
    }
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_subtypes(
            trig.source,
            grant,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
