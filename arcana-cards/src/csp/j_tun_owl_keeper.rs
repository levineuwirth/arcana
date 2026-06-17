//! Jötun Owl Keeper — `{2}{W}` 3/3 Giant.
//! Cumulative upkeep {W} or {U}; "When this creature dies, create a 1/1 white Bird
//! creature token with flying for each age counter on it."
//!
//! Cumulative upkeep is not a supported keyword and has no upkeep-tax cost shape —
//! GAP'd (the age counters it would accrue are not produced). The dies trigger is
//! wired: it creates one 1/1 white flying Bird per age counter currently on the
//! source (dynamic, read at resolution).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jötun Owl Keeper");
    let giant = reg.interner_mut().intern("Giant");
    let _bird = reg.interner_mut().intern("Bird");
    let _age = reg.interner_mut().intern("age");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Cumulative upkeep {W} or {U} — no upkeep-tax cost shape.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: make_birds,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_birds(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let age = match reg.interner().lookup("age") {
        Some(s) => CounterKind::Named(s),
        None => return Vec::new(),
    };
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(age));
    let bird = match reg.interner().lookup("Bird") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut out = Vec::new();
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(bird);
        out.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: bird,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        });
    }
    out
}
