//! Death-Greeter's Champion — `{2}{R}` 2/1 Human Warrior with Double strike.
//!
//! * Double strike — base keyword.
//! * Dash {3}{R} — alternative-cost keyword, not in the usable surface (GAP).
//! * Backup 1 — "When this creature enters, put a +1/+1 counter on target
//!   creature. If that's another creature, it gains [this creature's
//!   abilities] until end of turn." Modeled as an ETB trigger: add a +1/+1
//!   counter to the target; if the target is another creature, grant it
//!   Double strike (this card's granted ability) until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Death-Greeter's Champion");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    // GAP: Dash {3}{R} — alternative-cost keyword not in usable surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: backup_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn backup_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let target = *id;
    let mut effects = vec![Effect::AddCounters {
        target,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    // "If that's another creature, it gains [this creature's abilities]
    // until end of turn." This card's relevant granted ability is Double strike.
    if target != trig.source {
        effects.push(Effect::GrantKeyword {
            target,
            keyword: KeywordAbility::DoubleStrike,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
