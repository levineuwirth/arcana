//! Frontier Warmonger — `{3}{R}` 4/4 red Creature — Human Warrior.
//! "Whenever one or more creatures attack one of your opponents or a
//! planeswalker they control, those creatures gain menace until end of turn."
//! Wired via CreatureAttacks + trig.attacking_creature(): the trigger fires
//! once per attacker, granting menace to THAT creature when it attacks an
//! opponent or a planeswalker an opponent controls (per-creature firing of
//! the batched "one or more" trigger; same end state).

use arcana_core::combat::DefendingEntity;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::GameEvent;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Frontier Warmonger");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // Any creature attacking; the "one of your opponents or a
                // planeswalker they control" defender check happens in the
                // effect fn via the CreatureAttacks event.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: on_creature_attacks_grant_menace,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_creature_attacks_grant_menace(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Grant menace to THE attacking creature when it attacks an opponent
    // or a planeswalker an opponent controls.
    let Some(id) = trig.attacking_creature() else {
        return Vec::new();
    };
    let GameEvent::CreatureAttacks { defending, .. } = &trig.trigger_event else {
        return Vec::new();
    };
    let attacks_an_opponent = match defending {
        DefendingEntity::Player(p) => *p != trig.controller,
        DefendingEntity::Planeswalker(pw) => state
            .objects
            .get(*pw)
            .is_some_and(|o| o.controller != trig.controller),
        DefendingEntity::Battle(_) => false,
    };
    if !attacks_an_opponent {
        return Vec::new();
    }
    vec![Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Menace,
        duration: Duration::EndOfTurn,
    }]
}
