//! Vile Aggregate — `{2}{R}` */5 Eldrazi Drone (colorless, Devoid).
//!
//! Oracle:
//! * Devoid — the card is colorless (modeled as `colors: ColorSet::colorless()`;
//!   `Devoid` is not a modeled `KeywordAbility`).
//! * Vile Aggregate's power is equal to the number of colorless creatures you
//!   control — a characteristic-defining ability wired at Layer 7a via a
//!   `SelfEntersBattlefield` `self_pt_cda`. Power `*` resolves to the count of
//!   colorless creatures the controller has; toughness is the printed fixed 5.
//! * Trample — keyword.
//! * Ingest (Whenever this creature deals combat damage to a player, that player
//!   exiles the top card of their library.) — the `DamageDealt` trigger is
//!   wired, but the exile-top-of-library payload is GAP'd (no such Effect; Mill
//!   moves to graveyard, not exile).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vile Aggregate");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let self_name = reg.interner().lookup("Vile Aggregate");
    let self_filter = ObjectFilter { name: self_name, ..ObjectFilter::default() };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        // Devoid — colorless.
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // `*/5` — power is a CDA (`*` = colorless creatures you control),
        // toughness is the printed fixed 5. Resolved at Layer 7a by install_cda.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Ingest: whenever this creature deals combat damage to a player.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_filter,
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: ingest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Layer 7a self-CDA: power = colorless creatures you control, toughness = 5.
fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = number of colorless creatures you control; toughness = printed 5.
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| {
            o.controller == who
                && o.characteristics.types.is_creature()
                && o.characteristics.colors.is_colorless()
        })
        .count() as i32;
    (n, 5)
}

fn ingest(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "that player exiles the top card of their library" — there is no
    // exile-top-of-library Effect (Mill moves to graveyard, not exile).
    Vec::new()
}
