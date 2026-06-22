//! Hixus, Prison Warden — `{3}{W}{W}` 4/4 Legendary Human Soldier with Flash.
//! "Whenever a creature deals combat damage to you, if Hixus entered this
//! turn, exile that creature until Hixus leaves the battlefield."
//!
//! The combat-damage-to-you trigger is wired, but its body refers to
//! "that creature" — the creature that dealt the damage. The
//! `DamageDealt` event has no public accessor for its source object
//! (only `damage_amount`/`damaged_player`), and matching
//! `trig.trigger_event` directly is disallowed, so the exile body cannot
//! reach the damaging creature and is GAP'd (firing it without the right
//! target would be a materially wrong card). The intervening-if ("if
//! Hixus entered this turn") also has no `conditions::` predicate and is
//! GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hixus, Prison Warden");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: intervening-if "if Hixus entered this turn" — no
            // conditions:: predicate for source-entered-this-turn, so the
            // trigger fires whenever a creature deals combat damage to you.
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: exile_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn exile_attacker(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile that creature until Hixus leaves the battlefield" — the
    // DamageDealt event carries no public accessor for its source object
    // ("that creature"), so the exile target is unreachable. Emitting
    // ExileUntilSourceLeaves with the wrong/unknown target would be a
    // materially wrong card, so the body is omitted.
    Vec::new()
}
