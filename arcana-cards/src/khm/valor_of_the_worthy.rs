//! Valor of the Worthy — `{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +1/+1. When enchanted creature
//!  leaves the battlefield, create a 1/1 white Spirit creature token with
//!  flying."
//!
//! +1/+1 is an ETB-installed attached_pt. The leaves-the-battlefield payoff
//! is approximated by an `AttachedCreatureDoes { SelfDies }` host trigger
//! (only Self* conditions wrap; the broader "leaves" is narrowed to dies)
//! that creates a 1/1 white flying Spirit for the Aura's controller.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Valor of the Worthy");
    let aura = reg.interner_mut().intern("Aura");
    let _spirit = reg.interner_mut().intern("Spirit");
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
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // NOTE: "leaves the battlefield" narrowed to SelfDies (only
                // Self* conditions wrap into AttachedCreatureDoes).
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: on_host_leaves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_pump(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn on_host_leaves(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spirit = reg
        .interner()
        .lookup("Spirit")
        .expect("Spirit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let token = TokenDefinition {
        name: spirit,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
