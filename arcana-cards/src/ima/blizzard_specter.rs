//! Blizzard Specter — `{2}{U}{B}` 2/3 Snow Creature — Specter (U/B). Flying.
//! "Whenever this creature deals combat damage to a player, choose one —
//!  • That player returns a permanent they control to its owner's hand.
//!  • That player discards a card."
//!
//! Flying is a base keyword; Snow is a supertype. The combat-damage trigger is
//! modal, but modal dispatch is only available on spell abilities, not on
//! triggered abilities — so the choice is GAP'd along with the bounce mode; the
//! discard mode is expressed (the damaged player discards a card).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blizzard Specter");
    let specter = reg.interner_mut().intern("Specter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(specter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::SNOW),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: damaged_player_discards,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn damaged_player_discards(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one" modality (no modal dispatch on triggered abilities) and
    // the "returns a permanent they control to its owner's hand" mode (a
    // player-chosen bounce). Express the discard mode.
    let Some(p) = trig.damaged_player() else { return Vec::new(); };
    vec![Effect::Discard {
        player: p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
