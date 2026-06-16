//! Gideon Jura — `{3}{W}{W}` Legendary Planeswalker — Gideon, starting
//! loyalty 6.
//!
//! Loyalty abilities:
//! * `+2`: During target opponent's next turn, creatures that player
//!   controls attack Gideon Jura if able. (The "must attack this PW
//!   during a future turn" lure rider is not expressible from the
//!   demonstrated Effect surface — GAP'd; shell declared with the
//!   correct target + cost.)
//! * `−2`: Destroy target tapped creature.
//! * `0`: Until end of turn, Gideon Jura becomes a 6/6 Human Soldier
//!   creature that's still a planeswalker. Prevent all damage that would
//!   be dealt to him this turn. (PW ANIMATION — AddType(CREATURE) +
//!   SetBasePT(6/6) + PreventDamage on self; the "Human Soldier" subtype
//!   grant is not expressible — GAP'd.)
//!
//! # Rules references
//! * CR 606 — loyalty abilities; CR 704.5i — 0-loyalty sacrifice.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon Jura");
    let gideon = reg.interner_mut().intern("Gideon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gideon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    let tapped_creature = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::creature().tapped_only(),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: During target opponent's next turn, creatures that \
                       player controls attack Gideon Jura if able.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_lure,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Destroy target tapped creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![tapped_creature],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Gideon Jura becomes a 6/6 Human \
                       Soldier creature that's still a planeswalker. Prevent all \
                       damage that would be dealt to him this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_animate,
            }),
    )
}

fn plus_two_lure(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "during target opponent's next turn, that player's creatures
    // attack this planeswalker if able" — a future-turn forced-attack
    // (lure) rider is not expressible from the demonstrated Effect surface.
    Vec::new()
}

fn minus_two_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn zero_animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Human Soldier" creature-subtype grant is not expressible.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 6,
            toughness: 6,
            duration: Duration::EndOfTurn,
        },
        Effect::PreventDamage {
            target: DamageTarget::Object(ctx.source),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}
