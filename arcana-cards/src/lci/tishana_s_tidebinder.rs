//! Tishana's Tidebinder — `{2}{U}` 3/2 blue Merfolk Wizard with Flash.
//!
//! * Flash — keyword line.
//! * "When this creature enters, counter up to one target activated or
//!   triggered ability." → `SelfEntersBattlefield` trigger targeting
//!   `TargetFilter::AbilityOnStack` (activated or triggered, mana
//!   abilities never reach the stack) with `count: UpTo(1)`, resolving
//!   to `Effect::Counter`.
//! * GAP: "If an ability of an artifact, creature, or planeswalker is
//!   countered this way, that permanent loses all abilities for as long
//!   as this creature remains on the battlefield." — the resolver sees
//!   only the ability's stack-entry id, not its source permanent, and
//!   there is no countered-ability-source accessor; the lose-all-
//!   abilities rider can't be wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tishana's Tidebinder");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counter_ability,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::AbilityOnStack {
                        activated: true,
                        triggered: true,
                        source_filter: None,
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_counter_ability(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: the "that permanent loses all abilities while this creature
    // remains" rider is not expressible (no countered-ability-source
    // accessor); only the counter is emitted.
    vec![Effect::Counter { target: *id }]
}
