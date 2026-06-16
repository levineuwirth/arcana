//! Wrenn and Six — `{R}{G}` Legendary Planeswalker — Wrenn.
//! Starting loyalty inferred 3.
//! +1: Return up to one target land card from your graveyard to your hand.
//! −1: Wrenn and Six deals 1 damage to any target.
//! −7: You get an emblem with "Instant and sorcery cards in your graveyard
//!   have retrace."
//!
//! GAP: +1 targets a card in YOUR graveyard — there is no any-graveyard
//!   sentinel in the demonstrated target surface ("your graveyard" is
//!   controller-relative, not a fixed Zone::Graveyard(player)). Declared with
//!   the correct +1 cost, effect GAP'd.
//! GAP: −7 emblem grants the Retrace static to graveyard cards; emblem statics
//!   are not expressible (EmblemDefinition carries only triggered abilities).

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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn and Six");
    let wrenn = reg.interner_mut().intern("Wrenn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wrenn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Return up to one target land card from your graveyard to \
                       your hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_return_land,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Wrenn and Six deals 1 damage to any target.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"Instant and sorcery cards in your \
                       graveyard have retrace.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_return_land(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target land card from your graveyard" — controller-relative
    // graveyard targeting has no sentinel in the demonstrated surface.
    Vec::new()
}

fn minus_one_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => DamageTarget::Object(*id),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => DamageTarget::Player(*p),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 1,
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem static (Retrace grant to graveyard cards) not expressible.
    Vec::new()
}
