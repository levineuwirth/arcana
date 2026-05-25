//! Symbiote Spawn — `{2}{B}` 3/2 black Creature — Symbiote Villain.
//! "When this creature dies, each opponent loses 2 life and you gain 2 life."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Symbiote Spawn");
    let symbiote = reg.interner_mut().intern("Symbiote");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(symbiote);
    subtypes.0.insert(villain);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_drain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::LoseLife { player: opp, amount: 2 })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    effects
}
