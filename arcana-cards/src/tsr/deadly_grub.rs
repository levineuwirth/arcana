//! Deadly Grub — `{2}{B}` 3/1 black Insect with Vanishing 3.
//!
//! Vanishing 3 (enters with three time counters; remove one each
//! upkeep; sacrifice when the last is removed) is the parametrized
//! `KeywordAbility::Vanishing(3)` keyword — the engine synthesizes the
//! enter-with-counters / upkeep-removal / last-removed-sacrifice
//! machinery.
//!
//! "When this creature dies, if it had no time counters on it, create a
//! 6/1 green Insect creature token with shroud." → a `SelfDies` trigger
//! gated by an intervening-if that checks the source has no Time
//! counters, creating the token.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::effects::TokenDefinition;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deadly Grub");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vanishing(3)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: Some(if_no_time_counters),
                effect: dies_make_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_no_time_counters(
    s: &GameState,
    src: ObjectId,
    _you: arcana_core::types::PlayerId,
    _reg: &CardRegistry,
) -> bool {
    s.objects
        .get(src)
        .map_or(true, |o| o.count_counters(CounterKind::Time) == 0)
}

fn dies_make_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let insect = reg.interner().lookup("Insect").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: insect,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Shroud],
            abilities: vec![],
        },
    }]
}
