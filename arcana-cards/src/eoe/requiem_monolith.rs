//! Requiem Monolith — `{2}{B}` artifact.
//! "{T}: Until end of turn, target creature gains 'Whenever this creature
//! is dealt damage, you draw that many cards and lose that much life.'
//! That creature's controller may have this artifact deal 1 damage to it.
//! Activate only as a sorcery."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetChoice;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Requiem Monolith");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Until end of turn, target creature gains \
                       \"Whenever this creature is dealt damage, you draw \
                       that many cards and lose that much life.\" That \
                       creature's controller may have this artifact deal 1 \
                       damage to it. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_pain_bond,
            },
        ),
    )
}

/// "Until end of turn, target creature gains 'Whenever this creature is
/// dealt damage, you draw that many cards and lose that much life.'"
fn grant_pain_bond(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "That creature's controller may have this artifact deal 1 damage
    // to it" — a free may-choice by another player with no payment attached;
    // not expressible with OptionalPayment (Mana/Life only), so that clause
    // is omitted.
    vec![Effect::GrantTriggeredAbility {
        target: *id,
        ability: Box::new(TriggeredAbilityDef {
            id: GRANTED_TRIGGER_ID_BASE + 1,
            trigger_condition: TriggerCondition::SelfIsDealtDamage {
                combat_only: false,
            },
            intervening_if: None,
            effect: granted_draw_and_lose,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        duration: Duration::EndOfTurn,
    }]
}

/// Granted: "Whenever this creature is dealt damage, you draw that many
/// cards and lose that much life."
fn granted_draw_and_lose(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: n,
        },
        Effect::LoseLife {
            player: trig.controller,
            amount: n,
        },
    ]
}
