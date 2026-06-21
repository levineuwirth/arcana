//! Arahbo, Roar of the World — `{3}{G}{W}` 5/5 Legendary Cat Avatar.
//! Eminence — At the beginning of combat on your turn (whether in the
//! command zone or on the battlefield), another target Cat you control
//! gets +3/+3 until end of turn.
//! Whenever another Cat you control attacks, you may pay {1}{G}{W}. If
//! you do, it gains trample and gets +X/+X until end of turn, where X is
//! its power.
//!
//! Eminence is not a `KeywordAbility` variant — the keyword line is
//! empty. The two abilities are modeled as triggered abilities. The
//! command-zone-active half of Eminence cannot be expressed (the
//! demonstrated trigger only fires from the battlefield), so the trigger
//! only operates on the battlefield; that is a documented partial.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::actions::OptionalPaymentKind;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arahbo, Roar of the World");
    let cat = reg.interner_mut().intern("Cat");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(avatar);

    let cat_filter = script::subtype_filter(reg, "Cat").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Eminence — at the beginning of combat on your turn, another
            // target Cat you control gets +3/+3 until end of turn.
            // GAP: the "if Arahbo is in the command zone" half — the trigger
            // only fires from the battlefield in the demonstrated API.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: eminence_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(cat_filter.clone()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Whenever another Cat you control attacks, you may pay {1}{G}{W};
            // if you do, it gains trample and gets +X/+X (X = its power).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: cat_filter,
                },
                intervening_if: None,
                effect: roar_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn eminence_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn roar_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.attacking_creature() else { return Vec::new(); };
    let x = script::power_of(state, id).max(0);
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        then: Box::new(Effect::Pump {
            target: id,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample],
        }),
        else_effect: None,
    }]
}
