//! Moira and Teshar — `{3}{W}{B}` 4/5 Legendary Phyrexian Spirit Bird, Flying.
//! "Whenever you cast a historic spell, return target nonland permanent card
//! from your graveyard to the battlefield. It gains haste. Exile it at the
//! beginning of the next end step. If it would leave the battlefield, exile it
//! instead of putting it anywhere else."
//!
//! Partial: the "historic spell" restriction (artifact / legendary / Saga) is
//! not an expressible ObjectFilter, so the trigger fires on any spell you cast
//! (over-fires; noted as a GAP). The "if it would leave, exile instead" rider
//! is also unmodeled. The reanimate + haste + exile-at-next-end-step body is
//! expressed faithfully.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Moira and Teshar");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let spirit = reg.interner_mut().intern("Spirit");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(spirit);
    subtypes.0.insert(bird);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: "historic spell" filter (artifact/legendary/Saga) not
            // expressible; fires on any spell you cast.
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: reanimate_with_haste,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::permanent().without_types(TypeLine(
                        TypeLine::LAND | TypeLine::INSTANT | TypeLine::SORCERY,
                    )),
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_with_haste(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "if it would leave the battlefield, exile it instead" — replacement
    // rider unmodeled; the end-step exile is scheduled as a delayed action.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Exile,
        },
    ]
}
