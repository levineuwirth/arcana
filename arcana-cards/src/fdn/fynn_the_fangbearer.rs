//! Fynn, the Fangbearer — `{1}{G}` 1/3 Legendary Human Warrior.
//! Deathtouch.
//! Whenever a creature you control with deathtouch deals combat damage to a
//! player, that player gets two poison counters.
//!
//! Deathtouch is a base characteristic. The trigger condition (a deathtouch
//! creature you control dealing combat damage to a player) is wired faithfully
//! via DamageDealt; the effect ("that player gets two poison counters") is wired
//! via Effect::GivePlayerCounters { player: trig.damaged_player(), kind: Poison,
//! count: 2 } — the player-counter front-door over GameState::place_counters /
//! CounterTarget::Player.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fynn, the Fangbearer");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You)
                    .with_keyword(KeywordAbility::Deathtouch),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: poison_player,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn poison_player(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "that player gets two poison counters" — the damaged player gets 2 poison.
    let Some(player) = trig.damaged_player() else { return Vec::new(); };
    vec![Effect::GivePlayerCounters { player, kind: CounterKind::Poison, count: 2 }]
}
