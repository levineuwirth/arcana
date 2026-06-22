//! Dinosaur Egg — `{1}{G}` 0/3 green Dinosaur Egg.
//! "Evolve. When this creature dies, you may discover X, where X is its
//! toughness."

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Dinosaur Egg");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let egg = reg.interner_mut().intern("Egg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(egg);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        // Discover here is a keyword-shaped reminder on the dies trigger, not a
        // base keyword; only Evolve is a standing keyword.
        keywords: vec![KeywordAbility::Evolve],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_discover_toughness,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dies_discover_toughness(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // X is the dying creature's toughness (read its last-known toughness via the
    // dying object id). "You may" is a resolution-time choice; the engine prompt
    // is omitted, so Discover is performed unconditionally — close to faithful.
    let id = trig.dying_object().unwrap_or(trig.source);
    let x = script::toughness_of(state, id).max(0) as u32;
    vec![Effect::Discover {
        player: trig.controller,
        mana_value: x,
    }]
}
