//! Soulblade Corrupter — `{4}{B}` 3/3 Human Warrior.
//! Deathtouch.
//! Partner with Soulblade Renewer (ETB: target player may tutor Soulblade
//! Renewer to hand). — GAP: Partner-with tutor not modeled.
//! Whenever a creature with a +1/+1 counter on it attacks one of your
//! opponents, that creature gains deathtouch until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soulblade Corrupter");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    // GAP: Partner with Soulblade Renewer — the ETB "target player may put
    // Soulblade Renewer into their hand from their library" tutor-by-name is a
    // partner-linkage effect not expressible with the available primitives.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks {
                filter: ObjectFilter::creature(),
            },
            intervening_if: None,
            effect: grant_deathtouch_to_counter_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn grant_deathtouch_to_counter_attacker(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "a creature with a +1/+1 counter on it" — gate on the attacker having a
    // +1/+1 counter. (GAP fidelity: "attacks one of your opponents" — the
    // defending-opponent restriction is not separately checked.)
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    let has_counter = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne))
        > 0;
    if !has_counter {
        return Vec::new();
    }
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Deathtouch,
        duration: Duration::EndOfTurn,
    }]
}
