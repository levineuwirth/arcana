//! Glorious Enforcer — `{5}{W}{W}` 5/5 Creature — Angel with Flying and
//! Lifelink.
//!
//! Oracle:
//! * Flying, lifelink
//! * At the beginning of each combat, if you have more life than an opponent,
//!   this creature gains double strike until end of turn.
//!
//! GAP: the intervening-if "you have more life than an opponent" is a
//! comparison against opponent life totals; the demonstrated `conditions::`
//! helpers only compare your life to a fixed N, so the gate is not expressible.
//! Per guidance the if-clause is GAP'd and `intervening_if` is left `None`; the
//! double-strike grant fires at the beginning of each combat.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glorious Enforcer");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::Any,
            },
            // GAP: "if you have more life than an opponent" — not expressible.
            intervening_if: None,
            effect: gain_double_strike,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn gain_double_strike(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::DoubleStrike,
        duration: Duration::EndOfTurn,
    }]
}
