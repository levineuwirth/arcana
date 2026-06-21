//! Primeval Titan — `{4}{G}{G}` 6/6 Creature — Giant with Trample.
//!
//! Oracle:
//! * Trample — base keyword.
//! * Whenever this creature enters or attacks, you may search your library for
//!   up to two land cards, put them onto the battlefield tapped, then shuffle. —
//!   "enters or attacks" decomposes into two triggers (SelfEntersBattlefield +
//!   SelfAttacks). Each resolves "up to two land cards onto the battlefield
//!   tapped" as two `TutorToBattlefield` searches (the engine does one search
//!   per effect; repeating it twice models the up-to-two). Shuffle is automatic.
//!   The "up to" optionality is a minor fidelity gap (each search is best-effort
//!   and a no-op when no land is found).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Primeval Titan");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: fetch_two_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: fetch_two_lands,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Search your library for up to two land cards, put them onto the battlefield
/// tapped (two single-card searches), then shuffle (shuffle is automatic).
fn fetch_two_lands(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
    ]
}
