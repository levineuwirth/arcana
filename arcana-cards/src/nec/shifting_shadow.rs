//! Shifting Shadow — `{2}{R}` enchantment — Aura.
//! "Enchant creature. Enchanted creature has haste and 'At the beginning of
//!  your upkeep, destroy this creature. Reveal cards from the top of your
//!  library until you reveal a creature card. Put that card onto the
//!  battlefield and attach Shifting Shadow to it, then put all other cards
//!  revealed this way on the bottom of your library in a random order.'"
//!
//! The haste grant is an ETB-installed `attached_keyword(Haste)`. The granted
//! upkeep ability (destroy-self, reveal-until-creature, put-and-reattach) is
//! far beyond any expressible granted-ability primitive — GAP that clause.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Shifting Shadow");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
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
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted upkeep ability "destroy this creature; reveal until a
    // creature; put it onto battlefield and reattach Shifting Shadow; bottom
    // the rest" — far beyond expressible granted-ability primitives.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_keyword(
            trig.source,
            KeywordAbility::Haste,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
