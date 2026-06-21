//! Catti-brie of Mithral Hall — `{G}{W}` 2/2 Legendary Human Archer
//! with First strike and Reach.
//! "Whenever Catti-brie attacks, put a +1/+1 counter on it for each
//! Equipment attached to it." — GAP: there is no helper to count the
//! Equipment attached to a specific creature, so the per-Equipment
//! scaling can't be computed; the attack trigger is registered with a
//! GAP'd (empty) effect rather than emitting a fixed count.
//! "{1}, Remove all +1/+1 counters from Catti-brie: It deals X damage to
//! target attacking or blocking creature an opponent controls, where X
//! is the number of counters removed this way." — GAP: there is no
//! "remove ALL counters" activation cost (only a fixed count), and no
//! way to feed the removed amount into the damage as X, so the entire
//! activated ability is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Catti-brie of Mithral Hall");
    let human = reg.interner_mut().intern("Human");
    let archer = reg.interner_mut().intern("Archer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(archer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Reach],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_counters(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: +1/+1 counter "for each Equipment attached to it" — no helper
    // counts Equipment attached to a given creature, so the amount is
    // uncomputable.
    Vec::new()
}
