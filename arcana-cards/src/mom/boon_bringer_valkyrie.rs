//! Boon-Bringer Valkyrie — `{3}{W}{W}` 4/4 white Angel Warrior with Flying,
//! First strike, and Lifelink.
//! "Backup 1 (When this creature enters, put a +1/+1 counter on target
//! creature. If that's another creature, it gains the following abilities
//! until end of turn.)"

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
    let name = reg.interner_mut().intern("Boon-Bringer Valkyrie");
    let angel = reg.interner_mut().intern("Angel");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(warrior);
    // PARTIAL: "Backup 1" — Backup is not a supported KeywordAbility; the
    // mechanical core (ETB put a +1/+1 counter on target creature) is wired
    // below. GAP: the "if that's another creature, it gains [Flying, first
    // strike, lifelink] until end of turn" rider is not expressed (no
    // conditional self-vs-other ability grant in this shape).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
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

fn backup_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
