//! Trace of Abundance — `{R/W}{G}` enchantment — Aura.
//! "Enchant land.
//!  Enchanted land has shroud.
//!  Whenever enchanted land is tapped for mana, its controller adds an
//!  additional one mana of any color."
//!
//! Partial: the shroud grant is an ETB-installed `attached_keyword`. The
//! "tapped for mana → add additional mana" trigger is not expressible (no
//! Self* tapped-for-mana condition) and is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Trace of Abundance");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R/W}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::red() | ColorSet::green(),
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "whenever enchanted land is tapped for mana, add an additional mana of
    // any color" (no Self* tapped-for-mana trigger condition)
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Shroud,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
