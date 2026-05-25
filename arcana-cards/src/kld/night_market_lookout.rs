//! Night Market Lookout — `{B}` 1/1 black Human Rogue. "Whenever this creature
//! becomes tapped, each opponent loses 1 life and you gain 1 life."
//!
//! GAP: trigger — "becomes tapped" not in TriggerCondition catalog.
//! Using SelfAttacks as partial approximation (tapping also occurs on attacking).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Night Market Lookout");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "becomes tapped" not in catalog.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_tapped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_tapped(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: trigger — correct trigger is "becomes tapped", not attacks.
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife { player: opp, amount: 1 });
    }
    effects.push(Effect::GainLife { player: trig.controller, amount: 1 });
    effects
}
