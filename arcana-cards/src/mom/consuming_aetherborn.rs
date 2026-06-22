//! Consuming Aetherborn — `{3}{B}` 2/2 Aetherborn Vampire with Lifelink.
//!
//! * Backup 1 (When this creature enters, put a +1/+1 counter on target
//!   creature. If that's another creature, it gains the following ability
//!   until end of turn.)
//! * Lifelink.
//!
//! Backup is not in the usable KeywordAbility surface; the keyword itself is
//! GAP'd. Its expressible core — the ETB "+1/+1 counter on target creature" —
//! is wired as a triggered ability. The conditional grant ("if that's another
//! creature it gains Lifelink until end of turn") is GAP'd: there is no
//! "if the target isn't this creature, grant keyword" conditional primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::effects::KeywordAbility;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Consuming Aetherborn");
    let aetherborn = reg.interner_mut().intern("Aetherborn");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aetherborn);
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Backup 1 — ETB +1/+1 counter on target creature (expressible core).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: backup_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn backup_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "If that's another creature, it gains Lifelink until end of turn" —
    //       no conditional "if target != source then grant keyword" primitive.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
