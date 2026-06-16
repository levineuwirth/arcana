//! Animate Dead — `{1}{B}` enchantment — Aura.
//! "Enchant creature card in a graveyard
//!  When this Aura enters, … Return enchanted creature card to the
//!  battlefield under your control and attach this Aura to it. When this Aura
//!  leaves the battlefield, that creature's controller sacrifices it.
//!  Enchanted creature gets -1/-0."
//!
//! Reanimation with control change and a graveyard enchant target — neither
//! the graveyard-targeting enchant nor the return-under-your-control /
//! leaves-battlefield-sacrifice machinery is expressible. Only the -1/-0
//! attached_pt grant is wired (it applies once attached). The enchant target
//! is approximated by a battlefield creature.

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
    let name = reg.interner_mut().intern("Animate Dead");
    let aura = reg.interner_mut().intern("Aura");
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
        // NOTE: "enchant creature card in a graveyard" approximated by a
        // battlefield creature target (no graveyard-zone enchant filter).
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
    // GAP: return-creature-card-under-your-control reanimation + leaves-the-
    // battlefield-sacrifice — control-change/reanimation, not expressible.
    // Only the -1/-0 buff is wired.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(trig.source, -1, 0, Duration::WhileSourceOnBattlefield),
    }]
}
