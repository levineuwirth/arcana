//! Vampirism — `{1}{B}` black Enchantment—Aura.
//! "Enchant creature. When this Aura enters, draw a card at the beginning of
//! the next turn's upkeep. Enchanted creature gets +1/+1 for each other
//! creature you control. Other creatures you control get -1/-1."
//!
//! Partial: delayed draw + attached_pt_per_match (counting all your creatures,
//! not "other", as no self-exclude filter exists). GAP: other creatures -1/-1
//! sweep not expressible with an attached_* builder.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vampirism");
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
    // NOTE: "+1/+1 for each OTHER creature you control" — approximated as
    // all creatures you control (no self-exclude filter available); will
    // count one extra.
    // GAP: "other creatures you control get -1/-1" — no sweep ContinuousEffect
    // on attached_ builders; would need a global pump Effect which is wrong shape.
    vec![
        Effect::DelayedAction {
            source: trig.source,
            controller: trig.controller,
            when: DelayedWhen::NextUpkeep,
            action: DelayedAction::ControllerDrawsCard,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_pt_per_match(
                trig.source,
                ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                1,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
