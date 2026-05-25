//! Gloom Sower — `{5}{B}{B}` 8/6 Horror. "Whenever this creature becomes
//! blocked by a creature, that creature's controller loses 2 life and
//! you gain 2 life."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gloom Sower");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: on_becomes_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_becomes_blocked(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "that creature's controller" — we don't have the blocker id from
    // SelfBecomesBlocked; using opponents as approximation.
    let opponents = script::opponents(state, trig.controller);
    let mut effects = vec![Effect::GainLife { player: trig.controller, amount: 2 }];
    for p in opponents {
        effects.push(Effect::LoseLife { player: p, amount: 2 });
    }
    effects
}
