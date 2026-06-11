//! Duelist's Heritage — `{2}{W}` enchantment.
//! "Whenever one or more creatures attack, you may have target attacking
//! creature gain double strike until end of turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Duelist's Heritage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "whenever ONE OR MORE creatures attack"
                // fires once per attack event; CreatureAttacks fires once
                // PER attacking creature.
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature(),
                },
                intervening_if: None,
                effect: grant_double_strike,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                // "you may have target attacking creature…" — UpTo(1) lets
                // the controller decline. GAP: the "attacking" constraint
                // on the target has no demonstrated filter predicate.
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…gain double strike until end of turn."
fn grant_double_strike(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::DoubleStrike,
        duration: Duration::EndOfTurn,
    }]
}
