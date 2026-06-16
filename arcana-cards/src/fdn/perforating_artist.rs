//! Perforating Artist — `{1}{B}{R}` 3/2 Devil with Deathtouch. "Raid —
//! At the beginning of your end step, if you attacked this turn, each
//! opponent loses 3 life unless that player sacrifices a nonland
//! permanent of their choice or discards a card." The intervening-if
//! (you attacked this turn) is wired; the "lose 3 unless sac/discard"
//! payload uses a non-mana/non-life payment that is not expressible, so
//! the effect body is gapped.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Perforating Artist");
    let devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: Some(if_you_attacked),
            effect: raid_drain,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_you_attacked(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_attacked_this_turn(s, you)
}

fn raid_drain(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each opponent loses 3 life unless that player sacrifices a
    // nonland permanent or discards a card" — OptionalPayment only
    // supports Mana/Life payments, so the sacrifice-or-discard
    // alternative is not expressible. Omitted to avoid an unconditional
    // (materially wrong) life loss.
    Vec::new()
}
