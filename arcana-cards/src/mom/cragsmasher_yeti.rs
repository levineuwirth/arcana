//! Cragsmasher Yeti — `{4}{R}{R}` 4/2 Yeti.
//! Mountaincycling {2} (modeled as generic Cycling {2}; the type-search
//! variant is not separately modeled).
//! Backup 2 — When this creature enters, put two +1/+1 counters on target
//! creature. (The "if that's another creature it gains <ability>" rider is
//! GAP'd; Backup is not a stand-alone KeywordAbility.)
//! Trample.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cragsmasher Yeti");
    let yeti = reg.interner_mut().intern("Yeti");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(yeti);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: backup_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn backup_counters(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: Backup's "if that's another creature, it gains the following ability
    // until end of turn" rider — granting this creature's other abilities to the
    // counter recipient is not expressible (this card has no granted ability text).
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
