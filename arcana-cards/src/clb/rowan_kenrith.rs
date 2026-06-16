//! Rowan Kenrith — `{4}{R}{R}` Legendary Planeswalker — Rowan, starting loyalty
//! 5. Mono-red. (Partner with Will Kenrith; can be your commander — no rules
//! effect here; Partner/Partner-with not modeled.)
//!
//! +2: During target player's next turn, each creature that player controls
//!   attacks if able. GAP: a "during a future turn, force-attack" rider has no
//!   demonstrated surface. Ability shell declared with the +2 cost; effect GAP'd.
//! −2: Rowan Kenrith deals 3 damage to each tapped creature target player
//!   controls (`ForEach` over the target player's tapped creatures → DealDamage 3).
//! −8: Target player gets an emblem with an activated-ability copy rider. GAP:
//!   `EmblemDefinition` holds only statics + triggered abilities (no activated
//!   ability), and ability-copy isn't expressible. Ability shell declared; effect
//!   GAP'd (no emblem created — the grant cannot be built).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rowan Kenrith");
    let rowan = reg.interner_mut().intern("Rowan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rowan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
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
                text: "+2: During target player's next turn, each creature that \
                       player controls attacks if able.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Rowan Kenrith deals 3 damage to each tapped creature \
                       target player controls.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_burn_tapped,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Target player gets an emblem with \"Whenever you \
                       activate an ability that isn't a mana ability, copy it. \
                       You may choose new targets for the copy.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_gap,
            }),
    )
}

fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "during target player's next turn, each creature that player controls
    //      attacks if able" — no future-turn force-attack rider.
    Vec::new()
}

fn minus_two_burn_tapped(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else { return Vec::new(); };
    let tapped = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::Player(*p))
            .tapped_only(),
        ctx.controller,
    );
    tapped
        .into_iter()
        .map(|id| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 3,
        })
        .collect()
}

fn minus_eight_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the emblem grants an ability-copy TRIGGER on a target PLAYER; emblem
    //      ability-copy of activated abilities is not expressible.
    Vec::new()
}
