//! Ojer Pakpatiq, Deepest Epoch // Temple of Cyclical Time — {2}{U}{U}
//!
//! Front: Legendary Creature — God 4/3
//! Flying
//! Whenever you cast an instant spell from your hand, it gains rebound.
//! When Ojer Pakpatiq dies, return it to the battlefield tapped and transformed
//!   under its owner's control with three time counters on it.
//!
//! Back: Land
//! {T}: Add {U}. Remove a time counter from this land.
//! {2}{U}, {T}: Transform this land. Activate only if it has no time counters and only as sorcery.
//!
//! GAP: Rebound (instant gains rebound when cast from hand) — no Effect::GrantRebound variant.
//! GAP: "cast from your hand" qualifier on the trigger condition not modeled;
//!      SpellCast filter used for instant-type spells.
//! GAP: Dies trigger returning tapped+transformed with 3 time counters — "return tapped and
//!      transformed" from graveyard is not a single expressible effect; approximated as
//!      ReturnFromGraveyardToBattlefield + Transform + AddCounters (may have ordering issues).
//! GAP: Back-face land mana ability ({T}: Add {U}, remove time counter) not modeled.
//! GAP: Back-face activated transform ability ({2}{U},{T}: transform if no time counters) not modeled.
//! GAP: Back-face-only triggered/activated abilities not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ojer Pakpatiq, Deepest Epoch");
    let god_sub = reg.interner_mut().intern("God");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(god_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Temple of Cyclical Time");

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::LAND.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // When this creature dies, return it to battlefield tapped+transformed with 3 time counters
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: on_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
        // GAP: instant rebound grant trigger not modeled (no Effect::GrantRebound variant)
        // GAP: back-face land activations not modeled
    )
}

fn on_dies(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "return tapped and transformed with 3 time counters" from graveyard.
    // Best-effort approximation: return from graveyard, then transform, then add counters.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Time,
            count: 3,
        },
    ]
}
