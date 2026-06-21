//! Writhing Chrysalis — `{2}{R}{G}` 2/3 colorless (Devoid) Eldrazi
//! Drone with Reach.
//!
//! * Devoid → the card has no color (`ColorSet::colorless()`); Devoid is
//!   not a usable `KeywordAbility` variant, so it's reflected in the
//!   color set only, not the keyword line.
//! * "When you cast this spell, create two 0/1 colorless Eldrazi Spawn
//!   creature tokens with 'Sacrifice this token: Add {C}.'" → a
//!   `SpellCast` trigger (filtered to this card's name, firing from the
//!   Stack) creating two Eldrazi Spawn tokens. GAP: the token's
//!   "Sacrifice this token: Add {C}" activated ability can't be put on a
//!   `TokenDefinition` (its `abilities` field is triggered-only); the
//!   bare 0/1 token is minted.
//! * Reach — keyword line.
//! * "Whenever you sacrifice another Eldrazi, put a +1/+1 counter on
//!   this creature." → a `Sacrificed` trigger filtered to Eldrazi,
//!   adding a +1/+1 counter to itself.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Writhing Chrysalis");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let _spawn = reg.interner_mut().intern("Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let self_name = reg.interner().lookup("Writhing Chrysalis");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        name: self_name,
                        ..ObjectFilter::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_make_spawn,
                trigger_zones: vec![Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: script::subtype_filter(reg, "Eldrazi"),
                },
                intervening_if: None,
                effect: sac_eldrazi_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn cast_make_spawn(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Spawn").unwrap_or_default();
    let eldrazi = reg.interner().lookup("Eldrazi").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(spawn);
    // GAP: token's "Sacrifice this token: Add {C}" activated ability is
    // not representable on a TokenDefinition (abilities is triggered-only).
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}

fn sac_eldrazi_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
