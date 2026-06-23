//! Serra's Boon — `{2}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+2 as long as it's white.
//!  Otherwise, it gets -2/-1."
//!
//! Color-conditional P/T. An ETB-installed `attached_pt_dynamic` reads the
//! host (`source.attached_to`) and returns (+1, +2) when the enchanted
//! creature is white, else (-2, -1) — the "as long as it's white… otherwise…"
//! static, re-evaluated every layer application, so it tracks the host's
//! color changing.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serra's Boon");
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
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt_dynamic(
            trig.source,
            color_conditional_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn color_conditional_pt(state: &GameState, source: ObjectId) -> (i32, i32) {
    let Some(host) = state.object_or_lki(source).and_then(|o| o.attached_to) else {
        return (0, 0);
    };
    let Some(host_obj) = state.object_or_lki(host) else {
        return (0, 0);
    };
    if host_obj.characteristics.colors.0 & ColorSet::WHITE != 0 {
        (1, 2)
    } else {
        (-2, -1)
    }
}
