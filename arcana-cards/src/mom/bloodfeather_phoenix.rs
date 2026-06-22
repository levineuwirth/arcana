//! Bloodfeather Phoenix — `{1}{R}` 2/2 Phoenix (R) with Flying.
//!
//! * Flying
//! * This creature can't block. (Static combat restriction — GAP.)
//! * Whenever an instant or sorcery spell you control deals damage to
//!   an opponent or battle, you may pay {R}. If you do, return this
//!   card from your graveyard to the battlefield. It gains haste until
//!   end of turn. (Trigger fires from the graveyard; the "or battle"
//!   target alternative is narrowed to a player target.)

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodfeather Phoenix");
    let phoenix = reg.interner_mut().intern("Phoenix");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phoenix);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: static "This creature can't block."
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new()
                    .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY))
                    .controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: false,
            },
            intervening_if: None,
            effect: maybe_return,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn maybe_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{R}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![
            Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
            Effect::GrantKeyword {
                target: trig.source,
                keyword: KeywordAbility::Haste,
                duration: Duration::EndOfTurn,
            },
        ])),
        else_effect: None,
    }]
}
