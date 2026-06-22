//! Village Pillagers — `{3}{R}{R}` 5/5 Creature — Goblin Warrior. Wither.
//! "When this creature enters, it deals 1 damage to each creature your
//! opponents control."
//! "Whenever a creature an opponent controls with a counter on it dies, you
//! create a tapped Treasure token."
//!
//! Wither is a base keyword (the Scryfall "Treasure" keyword is just the token
//! it makes, not an ability keyword, and is dropped). The ETB sweep deals 1 to
//! each opposing creature via `ForEach`. The death trigger fires on an opponent
//! creature dying and creates a Treasure; the "with a counter on it" gate is
//! approximated by checking +1/+1 and -1/-1 counters (Wither's -1/-1 is the
//! relevant case) and the "tapped" rider on the Treasure is a fidelity GAP.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Village Pillagers");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Wither],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_damage_opp_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: counter_creature_dies_treasure,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_damage_opp_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    if targets.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 1,
        }),
    }]
}

fn counter_creature_dies_treasure(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Gate on "with a counter on it" — checked against the well-known +1/+1 and
    // -1/-1 counter kinds (Wither's -1/-1 is the relevant interaction).
    let Some(id) = trig.dying_object() else { return Vec::new(); };
    let had_counter = state.objects.get(id).map_or(false, |o| {
        o.count_counters(CounterKind::PlusOnePlusOne) > 0
            || o.count_counters(CounterKind::MinusOneMinusOne) > 0
    });
    if !had_counter {
        return Vec::new();
    }
    // GAP: the created Treasure should enter tapped — no tapped-token rider on
    // CreateCommodityToken; the Treasure is created untapped.
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Treasure,
        count: 1,
    }]
}
