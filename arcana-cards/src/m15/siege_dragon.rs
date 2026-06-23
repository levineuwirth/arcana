//! Siege Dragon — `{5}{R}{R}` 5/5 Creature — Dragon.
//!
//! Flying.
//! When this creature enters, destroy all Walls your opponents control.
//! Whenever this creature attacks, if defending player controls no Walls,
//! it deals 2 damage to each creature without flying that player controls.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Siege Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: destroy_opp_walls,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_sweep,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn destroy_opp_walls(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Wall").controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, trig.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}

fn attack_sweep(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(defender) = trig.defending_player() else {
        return Vec::new();
    };
    // Intervening-style gate evaluated against the defending player (an
    // event-specific player the conditions:: helpers cannot reach): the
    // sweep only happens if the defending player controls no Walls.
    let wall_filter =
        script::subtype_filter(reg, "Wall").controlled_by(ControllerConstraint::Opponent);
    if script::count_matching(state, &wall_filter, trig.controller) > 0 {
        return Vec::new();
    }
    // 2 damage to each creature without flying that player controls.
    let targets_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::Opponent)
        .without_keyword(KeywordAbility::Flying);
    let ids = script::ids_matching(state, &targets_filter, trig.controller);
    let _ = defender;
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 2,
        }),
    }]
}
