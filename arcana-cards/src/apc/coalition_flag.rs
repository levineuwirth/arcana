//! Coalition Flag — `{W}` enchantment — Aura.
//! "Enchant creature you control. Enchanted creature is a Flagbearer.
//! While an opponent is choosing targets ... that player must choose at
//! least one Flagbearer on the battlefield if able."
//!
//! ETB installs the added Flagbearer subtype. GAP: the Flagbearer
//! targeting rule (a static modification to opponents' target choices)
//! is not expressible.

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
    let name = reg.interner_mut().intern("Coalition Flag");
    let aura = reg.interner_mut().intern("Aura");
    let _flagbearer = reg.interner_mut().intern("Flagbearer");
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
        CardDefinition::new(name, chars)
            // NOTE: controller wording ("you control") approximated by caster's choice.
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the Flagbearer targeting rule (must-target-a-Flagbearer static).
    let mut flag = SubtypeSet::default();
    if let Some(f) = reg.interner().lookup("Flagbearer") {
        flag.0.insert(f);
    }
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_subtypes(
            trig.source,
            flag,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
