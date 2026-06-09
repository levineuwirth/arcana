//! Hagi Mob — `{4}{R}` 5/4 red Troll Berserker.
//! "Boast — {1}{R}: This creature deals 1 damage to any target.
//! (Activate only if this creature attacked this turn and only once each turn.)"
//!
//! The "attacked this turn" half of Boast is enforced via
//! `ActivationCost::activation_condition` + `conditions::source_attacked_this_turn`.
//! GAP: "only once each turn" — no per-turn activation counter on ActivationCost.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hagi Mob");
    let troll = reg.interner_mut().intern("Troll");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(berserker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Boast — {1}{R}: This creature deals 1 damage to any target.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    // Boast: only if this creature attacked this turn.
                    activation_condition: Some(|s, src, _you, _reg| {
                        arcana_core::conditions::source_attacked_this_turn(s, src)
                    }),
                    // GAP: "only once each turn" not expressible.
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

fn ping_any_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage { target: dt, amount: 1, source: ctx.source }]
}
