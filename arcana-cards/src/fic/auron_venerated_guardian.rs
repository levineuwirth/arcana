//! Auron, Venerated Guardian — `{3}{W}` 2/5 Legendary Creature —
//! Human Spirit Samurai with Vigilance.
//!
//! Oracle text:
//! * Vigilance — base keyword.
//! * "Shooting Star — Whenever Auron attacks, put a +1/+1 counter on it.
//!   When you do, exile target creature defending player controls with power
//!   less than Auron's power until Auron leaves the battlefield."
//!   The attack → +1/+1 counter on itself is modeled. The reflexive
//!   "when you do" sub-trigger (a second triggered ability gated on the
//!   first resolving, with a power-relative target filter "power less than
//!   Auron's power" + O-ring exile) is NOT expressible: there is no
//!   reflexive-trigger primitive and no power-relative-to-source filter.
//!   GAP'd; the counter half is emitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Auron, Venerated Guardian");
    let human = reg.interner_mut().intern("Human");
    let spirit = reg.interner_mut().intern("Spirit");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spirit);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_add_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_add_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "put a +1/+1 counter on it" — the reflexive exile half is GAP'd.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
