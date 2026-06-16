//! Ob Nixilis, the Hate-Twisted — `{3}{B}{B}` legendary planeswalker, starting loyalty 5.
//!
//! Static trigger: Whenever an opponent draws a card, Ob Nixilis deals 1
//!   damage to that player.
//! −2: Destroy target creature. Its controller draws two cards.
//!
//! Scope: both are fully expressed — the opponent-draw ping is a
//! battlefield triggered ability (read off the DrawCard event), and the
//! −2 destroys the target then makes its controller draw two.

use arcana_core::effects::Effect;
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ob Nixilis, the Hate-Twisted");
    let nixilis = reg.interner_mut().intern("Nixilis");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nixilis);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: ping_drawer,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Destroy target creature. Its controller draws two \
                       cards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_destroy_draw,
            }),
    )
}

/// "…deals 1 damage to that player" — read off the triggering DrawCard.
fn ping_drawer(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::DrawCard { player, .. } = trig.trigger_event else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(player),
        amount: 1,
    }]
}

/// `−2: Destroy target creature. Its controller draws two cards.`
fn minus_two_destroy_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let controller = script::target_controller(state, *id, ctx.controller);
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::DrawCards { player: controller, count: 2 },
    ]
}
