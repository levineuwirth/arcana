//! Basilica Stalker — `{5}{B}` 3/4 Creature — Vampire Detective with
//! Flying.
//!
//! Oracle:
//! * Flying.
//! * "Whenever this creature deals combat damage to a player, you gain
//!   1 life and surveil 1." — combat-damage-to-player trigger; the
//!   source filter is restricted to this card by name so only Basilica
//!   Stalker's own combat damage fires it.
//! * "Disguise {4}{B}" — GAP: Disguise is not in the usable keyword
//!   surface (no KeywordAbility::Disguise variant).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basilica Stalker");
    let vampire = reg.interner_mut().intern("Vampire");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(detective);

    let self_name = reg.interner().lookup("Basilica Stalker");
    let source_filter = ObjectFilter {
        name: self_name,
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter,
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: gain_and_surveil,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_and_surveil(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
        Effect::Surveil {
            player: trig.controller,
            count: 1,
        },
    ]
}
