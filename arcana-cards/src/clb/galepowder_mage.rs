//! Galepowder Mage — `{3}{W}` 3/3 Kithkin Wizard with Flying.
//!
//! * Flying (keyword).
//! * "Whenever this creature attacks, exile another target creature.
//!   Return that card to the battlefield under its owner's control at
//!   the beginning of the next end step." — a `SelfAttacks` trigger
//!   that exiles the target creature, then schedules a delayed return
//!   at the next end step (Vizier-of-Deferment idiom). GAP: the
//!   "another" self-exclusion on the target isn't expressible (no
//!   exclude-source target filter in the usable surface).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galepowder Mage");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: exile_until_end_step,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "another" — no exclude-source target filter; targets any creature.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn exile_until_end_step(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
