//! Pink Horror — `{3}{U}{R}` 4/4 Demon Horror.
//!
//! * Coruscating Flames — Whenever you cast an instant or sorcery
//!   spell, this creature deals 2 damage to any target. (`Coruscating
//!   Flames` is an ability-word, not a usable `KeywordAbility`; it is
//!   decomposed into the triggered ability below.)
//! * Split — When this creature dies, create two 2/2 blue and red Demon
//!   Horror creature tokens named Blue Horror with "Whenever you cast an
//!   instant or sorcery spell, this token deals 1 damage to any target."
//!   The tokens are minted, but their printed triggered ability is not
//!   expressible on a `TokenDefinition` (no demonstrated way to attach a
//!   triggered ability to a token) — GAP'd: bare tokens are created.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pink Horror");
    let demon = reg.interner_mut().intern("Demon");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: deal_two_any_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: split_into_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn deal_two_any_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 2,
    }]
}

fn split_into_tokens(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Blue Horror").unwrap_or_default();
    let demon = reg.interner().lookup("Demon").unwrap_or_default();
    let horror = reg.interner().lookup("Horror").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(horror);

    // GAP: the token's printed triggered ability ("Whenever you cast an
    // instant or sorcery spell, this token deals 1 damage to any target")
    // cannot be attached to a TokenDefinition; bare tokens are minted.
    let make_token = || Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::blue() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: subtypes.clone(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    };
    vec![make_token(), make_token()]
}
