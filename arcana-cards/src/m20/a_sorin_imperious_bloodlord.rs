//! A-Sorin, Imperious Bloodlord — `{2}{B}` legendary planeswalker,
//! starting loyalty 3. Subtype Sorin. (Alchemy rebalance variant.)
//!
//! # Loyalty abilities
//!
//! * `+1`: Target creature you control gains deathtouch and lifelink
//!   until end of turn. If it's a Vampire, put a +1/+1 counter on it.
//!   (Two GrantKeyword + a resolution-time conditional AddCounters.)
//! * `+1`: You may sacrifice a Vampire. When you do, Sorin deals 3
//!   damage to any target and you gain 3 life. GAP — optional
//!   sacrifice cost with a reflexive "when you do" trigger has no
//!   demonstrated activated-ability surface. Shell declared at +1.
//! * `−3`: You may put a Vampire creature card with mana value 6 or
//!   less from your hand onto the battlefield. GAP — filtered
//!   put-from-hand-to-battlefield is not in the demonstrated Effect
//!   surface. Shell declared at the correct −3 cost.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 / 606.3 — loyalty abilities.
//! * CR 704.5i — 0-loyalty state-based sacrifice.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Sorin, Imperious Bloodlord");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target creature you control gains deathtouch and \
                       lifelink until end of turn. If it's a Vampire, put a \
                       +1/+1 counter on it.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::You),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_buff,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You may sacrifice a Vampire. When you do, Sorin \
                       deals 3 damage to any target and you gain 3 life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_sac,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: You may put a Vampire creature card with mana \
                       value 6 or less from your hand onto the \
                       battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            }),
    )
}

/// `+1: Target creature you control gains deathtouch and lifelink until
/// end of turn. If it's a Vampire, put a +1/+1 counter on it.`
fn plus_one_buff(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let mut effects = vec![
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Deathtouch,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        },
    ];
    // "If it's a Vampire, put a +1/+1 counter on it."
    if let Some(vampire) = reg.interner().lookup("Vampire") {
        if let Some(obj) = state.objects.get(*id) {
            if obj.characteristics.subtypes.contains(vampire) {
                effects.push(Effect::AddCounters {
                    target: *id,
                    kind: CounterKind::PlusOnePlusOne,
                    count: 1,
                });
            }
        }
    }
    effects
}

/// `+1: You may sacrifice a Vampire. When you do, Sorin deals 3 damage
/// to any target and you gain 3 life.`
fn plus_one_sac(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional sacrifice cost ("you may sacrifice a Vampire") with a
    // reflexive "when you do" trigger is not expressible from the
    // demonstrated activated-ability surface.
    Vec::new()
}

/// `−3: You may put a Vampire creature card with mana value 6 or less
/// from your hand onto the battlefield.`
fn minus_three(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: filtered put-from-hand-onto-battlefield is not in the
    // demonstrated Effect surface.
    Vec::new()
}
