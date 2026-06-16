//! Giant Inheritance — `{4}{G}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +5/+5 and has 'Whenever this
//!  creature attacks, create a Monster Role token attached to up to one
//!  target attacking creature.' When this Aura is put into a graveyard
//!  from the battlefield, return it to its owner's hand."
//!
//! The +5/+5 is an ETB-installed `attached_pt`. The granted attack-trigger
//! (create a Role token attached to a target attacking creature) and the
//! leaves-battlefield return-to-hand payoff are not expressible.

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
    let name = reg.interner_mut().intern("Giant Inheritance");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: granted "whenever this creature attacks, create a Monster Role token
    // attached to up to one target attacking creature" host trigger and the
    // leaves-battlefield return-to-hand payoff are not expressible. +5/+5 only.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            5,
            5,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
