//! Spawn-Gang Commander — `{3}{R}{R}` 2/2 colorless Eldrazi Goblin (Devoid).
//!
//! Devoid (This card has no color.)
//! When you cast this spell, create three 0/1 colorless Eldrazi Spawn creature
//! tokens with "Sacrifice this token: Add {C}."
//! {1}{C}, Sacrifice an Eldrazi: This creature deals 2 damage to any target.
//!
//! Devoid is reflected by `colors: ColorSet::colorless()` (Scryfall lists Devoid
//! as a keyword but it carries no runtime rule beyond colorlessness).
//! The cast trigger mints three Eldrazi Spawn tokens; the tokens' own
//! "Sacrifice: Add {C}" activated ability is GAP'd (a TokenDefinition carries no
//! activated abilities in this surface).
//! The activated ping is wired with a "Sacrifice an Eldrazi" cost via
//! `sacrifice_other`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spawn-Gang Commander");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let goblin = reg.interner_mut().intern("Goblin");
    // Pre-intern the token's subtype so the resolver's lookup succeeds.
    let _spawn = reg.interner_mut().intern("Eldrazi Spawn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: arcana_core::targets::ControllerConstraint::You,
                },
                intervening_if: None,
                effect: cast_make_spawn,
                trigger_zones: vec![Zone::Stack],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{C}, Sacrifice an Eldrazi: This creature deals 2 damage to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{C}").expect("valid cost"),
                    sacrifice_other: Some(ObjectFilter::default().with_subtype_sym(eldrazi)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_any_target,
            }),
    )
}

fn cast_make_spawn(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Eldrazi Spawn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spawn);
    let token = TokenDefinition {
        name: spawn,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token's own "Sacrifice this token: Add {C}" activated ability — a
    // TokenDefinition carries no activated abilities in this surface.
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}

fn ping_any_target(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { source: ctx.source, target: dt, amount: 2 }]
}
