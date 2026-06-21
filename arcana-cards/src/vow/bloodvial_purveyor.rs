//! Bloodvial Purveyor — `{2}{B}{B}` 5/6 Creature — Vampire with Flying, Trample.
//!
//! * "Whenever an opponent casts a spell, that player creates a Blood token."
//!   — wired via `CreateCommodityToken { kind: Blood }` for the triggering
//!   caster.
//! * "Whenever this creature attacks, it gets +1/+0 until end of turn for each
//!   Blood token defending player controls." — wired: dynamic pump scaled by a
//!   `script::count_matching` of Blood tokens the defending player controls.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodvial Purveyor");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: opponent_makes_blood,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pump_per_blood,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn opponent_makes_blood(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    vec![Effect::CreateCommodityToken {
        controller: caster,
        kind: CommodityToken::Blood,
        count: 1,
    }]
}

fn attack_pump_per_blood(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(defender) = trig.defending_player() else {
        return Vec::new();
    };
    let blood = script::subtype_filter(reg, "Blood").controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &blood, defender) as i32;
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Pump {
        target: trig.source,
        power: n,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
