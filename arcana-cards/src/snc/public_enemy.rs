//! Public Enemy — `{2}{U}` enchantment — Aura.
//! "Enchant creature.
//!  All creatures attack enchanted creature's controller each combat if able.
//!  When enchanted creature dies, draw a card."
//!
//! Partial: the "all creatures attack enchanted creature's controller each
//! combat if able" board-wide attack-forcing is not expressible and is omitted.
//! The "when enchanted creature dies, draw a card" payoff IS a host trigger
//! (`AttachedCreatureDoes { SelfDies }`); the Aura's controller draws.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Public Enemy");
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
            // GAP: "all creatures attack enchanted creature's controller each combat if able"
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: draw_on_host_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_on_host_dies(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(controller) = state.object_or_lki(trig.source).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DrawCards { player: controller, count: 1 }]
}
