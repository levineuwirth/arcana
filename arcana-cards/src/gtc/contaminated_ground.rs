//! Contaminated Ground — `{1}{B}` enchantment — Aura.
//! "Enchant land. Enchanted land is a Swamp. Whenever enchanted land
//!  becomes tapped, its controller loses 2 life."
//!
//! ETB installs an `attached_subtypes` making the host a Swamp. A host
//! `AttachedCreatureDoes(SelfBecomesTapped)` trigger makes the host's
//! controller lose 2 life (host reached via `source.attached_to`).

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
    let name = reg.interner_mut().intern("Contaminated Ground");
    let aura = reg.interner_mut().intern("Aura");
    // Interned here so the ETB effect fn can `lookup` it.
    let _swamp = reg.interner_mut().intern("Swamp");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
                effect: etb_make_swamp,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesTapped),
                },
                intervening_if: None,
                effect: on_host_tapped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_make_swamp(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let swamp = reg.interner().lookup("Swamp").expect("Swamp interned");
    let mut s = SubtypeSet::default();
    s.0.insert(swamp);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_subtypes(
            trig.source,
            s,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn on_host_tapped(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(src) = state.objects.get(trig.source) else {
        return Vec::new();
    };
    let Some(host) = src.attached_to else {
        return Vec::new();
    };
    let Some(host_obj) = state.objects.get(host) else {
        return Vec::new();
    };
    vec![Effect::LoseLife {
        player: host_obj.controller,
        amount: 2,
    }]
}
