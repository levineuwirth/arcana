//! Jiwari, the Earth Aflame — `{3}{R}{R}` 3/3 Legendary Spirit.
//!
//! Oracle:
//! * "{X}{R}, {T}: Jiwari deals X damage to target creature without
//!   flying." — a tap + {X}{R} activated ability dealing X damage to a
//!   non-flying creature. X is read from `ctx.x_value`; generic-{X}
//!   threading into `x_value` is a documented fidelity gap (it may be 0 if
//!   the engine doesn't thread it for non-loyalty {X}).
//! * "Channel — {X}{R}{R}{R}, Discard this card: It deals X damage to each
//!   creature without flying." — a Channel ability (mana + discard-self
//!   from hand) dealing X damage to every non-flying creature via a
//!   ForEach over the matching ids.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jiwari, the Earth Aflame");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{R}, {T}: Jiwari deals X damage to target creature without flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().without_keyword(KeywordAbility::Flying),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: damage_target_nonflyer,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Channel — {X}{R}{R}{R}, Discard this card: It deals X damage to each \
                       creature without flying."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{R}{R}{R}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: channel_damage_each_nonflyer,
            }),
    )
}

fn damage_target_nonflyer(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: x,
    }]
}

fn channel_damage_each_nonflyer(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let filter = ObjectFilter::creature().without_keyword(KeywordAbility::Flying);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: x,
        }),
    }]
}
