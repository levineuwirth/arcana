//! Jeskai Runemark — `{2}{U}` enchantment — Aura (Fate Reforged).
//! "Enchant creature. Enchanted creature gets +2/+2. Enchanted creature
//!  has flying as long as you control a red or white permanent."
//!
//! The +2/+2 buff is an ETB-installed `attached_pt`. The conditional
//! flying ("as long as you control a red or white permanent") is a
//! state-conditional static grant that the demonstrated API can't gate,
//! so it is GAP'd; the unconditional +2/+2 is honored.

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
    let name = reg.interner_mut().intern("Jeskai Runemark");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
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
    // GAP: conditional flying — "has flying as long as you control a red or
    // white permanent" is a state-gated static grant the API can't express.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(trig.source, 2, 2, Duration::WhileSourceOnBattlefield),
    }]
}
