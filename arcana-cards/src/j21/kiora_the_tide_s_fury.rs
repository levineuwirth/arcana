//! Kiora, the Tide's Fury — `{3}{U}` legendary planeswalker, starting loyalty 7.
//!
//! +1: Conjure a card named Kraken Hatchling into your hand (Conjure GAP).
//! +1: Untap target creature or land. Prevent all damage to/by it until
//!     your next turn (prevention rider GAP).
//! −3: You may sacrifice a Kraken. If you do, create an 8/8 blue Kraken
//!     token (may/if-you-do conditional GAP).
//!
//! Scope: the second +1 expresses the Untap of its target; the damage-
//! prevention rider has no demonstrated targeted-prevention Effect. The
//! first +1 (Conjure) and the −3 (optional-sacrifice-then-create
//! conditional) are GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kiora, the Tide's Fury");
    let kiora = reg.interner_mut().intern("Kiora");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kiora);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    // "target creature or land"
    let cre_or_land = ObjectFilter::permanent()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::LAND));

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Conjure a card named Kraken Hatchling into your hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_conjure,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap target creature or land. Prevent all damage \
                       that would be dealt to and dealt by that permanent until \
                       your next turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(cre_or_land),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: You may sacrifice a Kraken. If you do, create an 8/8 \
                       blue Kraken creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_kraken,
            }),
    )
}

/// First `+1` — Conjure.
fn plus_one_conjure(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: Conjure has no demonstrated Effect.
    Vec::new()
}

/// Second `+1` — untap the target (damage-prevention rider GAP'd).
fn plus_one_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "prevent all damage to and by that permanent until your next
    // turn" has no demonstrated targeted-prevention Effect.
    vec![Effect::Untap { target: *id }]
}

/// `−3` — optional sacrifice then conditional create.
fn minus_three_kraken(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may sacrifice ... if you do, create ..." (optional cost
    // gating a follow-up) not expressible from the demonstrated surface.
    Vec::new()
}
