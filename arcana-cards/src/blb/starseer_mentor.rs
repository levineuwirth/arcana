//! Starseer Mentor — `{3}{W}{B}` 3/5 Bat Warlock with Flying and Vigilance.
//! "At the beginning of your end step, if you gained or lost life this turn,
//! target opponent loses 3 life unless they sacrifice a nonland permanent of
//! their choice or discard a card."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::ObjectId;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::conditions;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Starseer Mentor");
    let bat = reg.interner_mut().intern("Bat");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_life_changed),
            // GAP: "unless they sacrifice a nonland permanent or discard a card"
            // is a payer-chosen either/or alternative that OptionalPayment can't
            // express (only Mana/Life). The whole resolution is GAP'd.
            effect: starseer_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn if_life_changed(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    // "gained or lost life this turn" — only the gained-life half has a helper;
    // the lost-life half is a partial.
    conditions::you_gained_life_this_turn(s, you)
}

fn starseer_drain(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "loses 3 life unless they sacrifice a nonland permanent of their choice
    // or discard a card" — opponent-chosen either/or alternative not expressible.
    Vec::new()
}
