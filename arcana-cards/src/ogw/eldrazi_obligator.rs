//! Eldrazi Obligator — `{2}{R}` 3/1 Eldrazi (Devoid — colorless).
//!
//! Devoid.
//! When you cast this spell, you may pay {1}{C}. If you do, gain control of
//! target creature until end of turn, untap that creature, and it gains haste
//! until end of turn.
//! Haste.
//!
//! Devoid makes the card colorless (it is not a usable KeywordAbility); Haste
//! is a base keyword. The cast trigger is filtered by name so it fires only for
//! this spell; on optional {1}{C} payment it takes control of the target
//! creature until end of turn, untaps it, and grants it haste.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eldrazi Obligator");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let self_name = reg.interner_mut().intern("Eldrazi Obligator");
    let self_filter = ObjectFilter {
        name: Some(self_name),
        ..ObjectFilter::default()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(self_filter),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: cast_obligate,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_creature()],
        }),
    )
}

fn cast_obligate(
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
    let id = *id;
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{C}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![
            Effect::ChangeControlEot {
                target: id,
                new_controller: trig.controller,
            },
            Effect::Untap { target: id },
            Effect::GrantKeyword {
                target: id,
                keyword: KeywordAbility::Haste,
                duration: Duration::EndOfTurn,
            },
        ])),
        else_effect: None,
    }]
}
