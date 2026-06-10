//! Cabal Pit — nonbasic land. "{T}: Add {B}. This land deals 1
//! damage to you." and "Threshold — {B}, {T}, Sacrifice this land:
//! Target creature gets -2/-2 until end of turn. Activate only if
//! there are seven or more cards in your graveyard."
//!
//! The threshold activation gate ("Activate only if there are seven
//! or more cards in your graveyard") is not expressible — see the
//! GAP comment.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cabal Pit");
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
                text: "{T}: Add {B}. This land deals 1 damage to you.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black_mana_ping_self,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Threshold — {B}, {T}, Sacrifice this land: Target \
                       creature gets -2/-2 until end of turn. Activate only \
                       if there are seven or more cards in your graveyard."
                    .into(),
                // GAP: "Activate only if there are seven or more cards in your
                // graveyard" (threshold) — no activation-precondition field exists
                // in the ActivationCost catalog; the ability is activatable
                // without the graveyard check.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: shrink_target,
            }),
    )
}

/// CR 605.1a mana ability with a damage rider — the {B} plus
/// "this land deals 1 damage to you".
fn add_black_mana_ping_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
        },
        Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
            source: ctx.source,
        },
    ]
}

fn shrink_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: -2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
