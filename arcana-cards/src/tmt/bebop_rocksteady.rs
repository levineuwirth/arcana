//! Bebop & Rocksteady — `{1}{B/G}{B/G}` 7/5 black-green Legendary Creature —
//! Boar Rhino Mutant.
//! "Whenever Bebop & Rocksteady attack or block, sacrifice a permanent unless
//! you discard a card."
//! GAP: "unless you discard a card" optional choice not expressible;
//! emitting Sacrifice unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bebop & Rocksteady");
    let boar = reg.interner_mut().intern("Boar");
    let rhino = reg.interner_mut().intern("Rhino");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    subtypes.0.insert(rhino);
    subtypes.0.insert(mutant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B/G}{B/G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attack_or_block_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: on_attack_or_block_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attack_or_block_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "unless you discard a card" optional choice not expressible
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::permanent(),
        count: 1,
    }]
}
