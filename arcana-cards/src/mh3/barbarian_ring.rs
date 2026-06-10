//! Barbarian Ring — nonbasic land.
//! "{T}: Add {R}. This land deals 1 damage to you." and
//! "Threshold — {R}, {T}, Sacrifice this land: It deals 2 damage to
//! any target. Activate only if there are seven or more cards in your
//! graveyard."
//! The mana ability carries its printed self-damage rider (CR 605.1a —
//! an ability with a rider that adds mana is still a mana ability).
//! GAP: the Threshold activation precondition ("Activate only if there
//! are seven or more cards in your graveyard") is not expressible —
//! ActivationCost has no graveyard-count gate; the ability is
//! activatable unconditionally.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barbarian Ring");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}. This land deals 1 damage to you.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_ping_self,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Threshold — {R}, {T}, Sacrifice this land: It deals 2 damage to any target. Activate only if there are seven or more cards in your graveyard.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: threshold_bolt,
            }),
    )
}

fn add_red_ping_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
        },
        Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
            source: ctx.source,
        },
    ]
}

fn threshold_bolt(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'Activate only if there are seven or more cards in your
    // graveyard' — no activation precondition field in the catalog.
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(op) => match op {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { target: dt, amount: 2, source: ctx.source }]
}
