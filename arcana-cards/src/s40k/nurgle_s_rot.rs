//! Nurgle's Rot — `{B}` enchantment — Aura.
//! "Enchant creature an opponent controls. When enchanted creature dies,
//! return this card to its owner's hand and you create a 1/3 black Demon
//! creature token named Plaguebearer of Nurgle."
//!
//! Host death trigger (`AttachedCreatureDoes { SelfDies }`): returns this
//! Aura (`trig.source`) to its owner's hand and creates the 1/3 black
//! Demon token. The "enchant creature an opponent controls" restriction
//! is honored by the caster's choice.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Nurgle's Rot");
    let aura = reg.interner_mut().intern("Aura");
    let _plaguebearer = reg.interner_mut().intern("Plaguebearer of Nurgle");
    let _demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // NOTE: "an opponent controls" honored by caster's choice.
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: |_, _, _| Vec::new(),
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: on_host_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_host_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::ReturnToHand { target: trig.source }];
    if let (Some(pname), Some(demon)) = (
        reg.interner().lookup("Plaguebearer of Nurgle"),
        reg.interner().lookup("Demon"),
    ) {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(demon);
        let token = TokenDefinition {
            name: pname,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        };
        effects.push(Effect::CreateToken { controller: trig.controller, token });
    }
    effects
}
