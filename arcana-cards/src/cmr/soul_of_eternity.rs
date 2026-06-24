//! Soul of Eternity — `{5}{W}{W}` */* Avatar with Encore {7}{W}{W}.
//!
//! Soul of Eternity's power and toughness are each equal to your life total.
//! Encore {7}{W}{W} ({7}{W}{W}, Exile this card from your graveyard: For each
//! opponent, create a token copy that attacks that opponent this turn if able.
//! They gain haste. Sacrifice them at the beginning of the next end step.
//! Activate only as a sorcery.)
//!
//! The */* CDA ("P/T equal to your life total") is recorded with
//! `PtValue::Star`; the self-CDA is installed at Layer 7a via
//! `ContinuousEffect::self_pt_cda` on a `SelfEntersBattlefield` trigger.
//! Encore is not a `KeywordAbility` variant; it is modeled as a
//! graveyard-activated ability with cost {7}{W}{W} + exile-self, but its
//! effect (per-opponent attacking token copies, haste, end-step sacrifice,
//! sorcery-speed) cannot be assembled from the available primitives and is
//! GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul of Eternity");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
            text: "Encore {7}{W}{W} ({7}{W}{W}, Exile this card from your graveyard: For each opponent, create a token copy that attacks that opponent this turn if able. They gain haste. Sacrifice them at the beginning of the next end step. Activate only as a sorcery.)".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{7}{W}{W}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: encore_effect,
        }),
    )
}

/// "Soul of Eternity's power and toughness are each equal to your life total"
/// — install the self-CDA at Layer 7a.
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            life_total_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// P/T = the controller's life total.
fn life_total_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s.player(who).life;
    (n, n)
}

fn encore_effect(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Encore body — per-opponent token copies that attack that opponent,
    // gain haste, and are sacrificed at the next end step is not expressible
    // (no "token copy of a graveyard card that attacks a specific opponent"
    // primitive).
    Vec::new()
}
