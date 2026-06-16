//! Sedraxis Specter — `{U}{B}{R}` 3/2 Specter with Flying.
//! "Whenever this creature deals combat damage to a player, that player
//!  discards a card.
//!  Unearth {1}{B}."

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sedraxis Specter");
    let specter = reg.interner_mut().intern("Specter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(specter);

    // Unearth {1}{B} is a graveyard-activation keyword not on the usable keyword
    // surface — GAP'd (no Unearth KeywordAbility variant).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::permanent(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_discard,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    vec![Effect::Discard {
        player: p,
        count: 1,
        choice: DiscardChoice::ControllerChooses,
    }]
}
