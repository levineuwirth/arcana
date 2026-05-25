//! Scheming Aspirant — `{1}{B}` 1/3 Phyrexian Advisor. "Whenever
//! you proliferate, each opponent loses 2 life and you gain 2 life."
//!
//! GAP: trigger — the engine's `TriggerCondition` catalog has no
//! "whenever you proliferate" variant. The closest available shape
//! is picked below as a placeholder so the file compiles; verify
//! will quarantine this card until a `Proliferated` trigger lands.
//! The effect fn itself (each-opponent-loses-2, you-gain-2) is
//! authored faithfully so it runs when the placeholder fires.
//!
//! GAP: keyword — `Proliferate` is an action word that appears in
//! oracle text rather than a static keyword on the card; the
//! `KeywordAbility` enum does not include it, so `keywords` is
//! empty.

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
    let name = reg.interner_mut().intern("Scheming Aspirant");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever you proliferate" has no
                // matching `TriggerCondition` variant; using
                // `SelfEntersBattlefield` as the closest no-arg
                // placeholder so verify can flag the gap.
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_proliferate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Each opponent loses 2 life and you gain 2 life."
fn on_proliferate(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 2 })
        .collect();
    effects.push(Effect::GainLife { player: trig.controller, amount: 2 });
    vec![Effect::Sequence(effects)]
}
