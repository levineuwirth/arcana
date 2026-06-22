//! Gwaihir, Greatest of the Eagles — `{4}{W}` 5/5 Legendary Creature —
//! Bird Noble with Flying.
//! "Whenever Gwaihir attacks, target attacking creature gains flying until
//! end of turn."
//! "At the beginning of each end step, if you gained 3 or more life this
//! turn, create a 3/3 white Bird creature token with flying and 'Whenever
//! this token attacks, target attacking creature gains flying until end of
//! turn.'"
//!
//! Both triggers are wired. The end-step token's "if you gained 3 or more
//! life this turn" gate has no conditions:: predicate and is GAP'd to None
//! (so it fires each end step).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gwaihir, Greatest of the Eagles");
    let bird = reg.interner_mut().intern("Bird");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(noble);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: grant_flying_to_attacker,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![attacking_creature_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                // GAP: intervening-if "if you gained 3 or more life this
                //      turn" — no conditions:: predicate for life gained
                //      this turn; left None so the body fires each end step.
                intervening_if: None,
                effect: make_bird_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attacking_creature_target() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::creature().attacking_only()),
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

fn grant_flying_to_attacker(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}

fn make_bird_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bird = reg.interner().lookup("Bird").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    // The token's own attack trigger mirrors Gwaihir's first ability.
    let token_attack = TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfAttacks,
        intervening_if: None,
        effect: grant_flying_to_attacker,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![attacking_creature_target()],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: bird,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![token_attack],
        },
    }]
}
