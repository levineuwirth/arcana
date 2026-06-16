//! Rowan, Fearless Sparkmage — `{3}{R}{R}` Legendary Planeswalker — Rowan.
//! Starting loyalty inferred 5.
//! +1: Up to one target creature gets +3/+0 and gains first strike until end
//!   of turn.
//! −2: Rowan deals 1 damage to each of up to two target creatures. Those
//!   creatures can't block this turn.
//! −9: Gain control of all creatures until end of turn. Untap them. They gain
//!   haste until end of turn.
//!
//! GAP: −2 "those creatures can't block this turn" — there is no can't-block
//!   continuous effect in the demonstrated surface (only CantBeBlocked, which
//!   is the inverse). The 1 damage to each target is modeled; the can't-block
//!   rider is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rowan, Fearless Sparkmage");
    let rowan = reg.interner_mut().intern("Rowan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rowan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets +3/+0 and gains first strike \
                       until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Rowan deals 1 damage to each of up to two target creatures. \
                       Those creatures can't block this turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: Gain control of all creatures until end of turn. Untap them. \
                       They gain haste until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_control,
            }),
    )
}

fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 3,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike],
    }]
}

fn minus_two_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "those creatures can't block this turn" — no can't-block effect.
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::DealDamage {
                source: ctx.source,
                target: DamageTarget::Object(*id),
                amount: 1,
            }),
            _ => None,
        })
        .collect()
}

fn minus_nine_control(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature();
    let ids = script::ids_matching(state, &filter, ctx.controller);
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::ChangeControlEot {
            target: id,
            new_controller: ctx.controller,
        });
        effects.push(Effect::Untap { target: id });
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
