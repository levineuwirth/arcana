//! Maestros Diabolist — `{U}{B}{R}` 1/4 Vampire Warrior.
//!
//! * Deathtouch, Haste (keywords).
//! * "Whenever this creature attacks, if you don't control a Devil
//!   token, create a tapped and attacking 1/1 red Devil creature token
//!   with 'When this token dies, it deals 1 damage to any target.'"
//!   (SelfAttacks trigger; intervening-if = control no Devil token;
//!   `CreateTokenTappedAttacking` with an on-death any-target ping.)

use arcana_core::effects::Effect;
use arcana_core::effects::{KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maestros Diabolist");
    let vampire = reg.interner_mut().intern("Vampire");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: Some(if_no_devil_token),
                effect: make_devil_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_no_devil_token(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let devil_tokens = script::subtype_filter(reg, "Devil")
        .controlled_by(ControllerConstraint::You)
        .tokens_only();
    arcana_core::conditions::you_control_at_most(s, you, &devil_tokens, 0)
}

fn make_devil_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let devil = reg.interner().lookup("Devil").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil);
    let token = TokenDefinition {
        name: devil,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfDies,
            intervening_if: None,
            effect: devil_death_ping,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::any_target()],
        }],
    };
    vec![Effect::CreateTokenTappedAttacking {
        controller: trig.controller,
        token,
    }]
}

fn devil_death_ping(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 1,
    }]
}
