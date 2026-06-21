//! Malcolm, Keen-Eyed Navigator — `{2}{U}` 2/2 Legendary Siren Pirate
//! with Flying.
//! "Whenever one or more Pirates you control deal damage to your
//! opponents, you create a Treasure token for each opponent dealt
//! damage."
//! "Partner."
//!
//! Flying is a base keyword. Partner is a commander-format keyword with
//! no `KeywordAbility` variant — GAP'd. The Pirate-damage trigger fires
//! on a Pirate you control dealing damage to a player and mints one
//! Treasure. The exact "one or more … for each opponent dealt damage"
//! batching (one trigger per combat, counting distinct opponents) is a
//! fidelity gap — this fires per damage event and creates a single
//! Treasure.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malcolm, Keen-Eyed Navigator");
    let siren = reg.interner_mut().intern("Siren");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siren);
    subtypes.0.insert(pirate);
    let pirate_filter = script::subtype_filter(reg, "Pirate")
        .controlled_by(ControllerConstraint::You);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP (Partner): commander-format keyword with no KeywordAbility variant.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: pirate_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: false,
                },
                intervening_if: None,
                effect: make_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_treasure(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
