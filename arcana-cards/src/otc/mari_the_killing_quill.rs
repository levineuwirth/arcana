//! Mari, the Killing Quill — `{1}{B}{B}` 3/2 Legendary Vampire Assassin.
//!
//! * Whenever a creature an opponent controls dies, exile it with a hit
//!   counter on it.
//! * Assassins, Mercenaries, and Rogues you control have deathtouch and a
//!   combat-damage triggered ability that spends hit counters. (GAP'd —
//!   pure static continuous ability granting keywords + a triggered
//!   ability to a group of your creatures.)
//!
//! "Treasure" on the keyword line is a Scryfall mechanic, not a
//! `KeywordAbility` variant, so the keywords vec is empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mari, the Killing Quill");
    let vampire = reg.interner_mut().intern("Vampire");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(assassin);
    let _ = reg.interner_mut().intern("hit");

    let opponent_creatures =
        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: opponent_creatures,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: exile_with_hit_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: "Assassins, Mercenaries, and Rogues you control have
        // deathtouch and '<combat-damage hit-counter trigger>'." — pure
        // static continuous group grant of a keyword + triggered ability.
    )
}

/// Exile the dying opposing creature with a hit counter on it.
fn exile_with_hit_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else { return Vec::new(); };
    let Some(hit) = reg.interner().lookup("hit").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::Sequence(vec![
        Effect::ExileFromGraveyard { target: id },
        Effect::AddCounters { target: id, kind: hit, count: 1 },
    ])]
}
