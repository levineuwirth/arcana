//! Tezzeret, Cruel Machinist — `{4}{U}{U}` Legendary Planeswalker — Tezzeret,
//! starting loyalty 5.
//!
//! +1: Draw a card.
//! 0: Until your next turn, target artifact you control becomes a 5/5 creature
//!    in addition to its other types.
//! −7: Put any number of cards from your hand onto the battlefield face down.
//!     They're 5/5 artifact creatures.
//!
//! GAP: −7 "put any number of cards from your hand onto the battlefield face
//!   down as 5/5 artifact creatures" — there is no Effect variant for putting
//!   face-down manifest/morph-style permanents from hand with a fixed P/T.
//!   The ability shell is declared with the correct cost; effect returns
//!   Vec::new().

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret, Cruel Machinist");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    let artifact_you = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw a card.".into(),
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
                effect: plus_one_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until your next turn, target artifact you control \
                       becomes a 5/5 creature in addition to its other types."
                    .into(),
                cost: ActivationCost::default(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(artifact_you),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_becomes_creature,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Put any number of cards from your hand onto the \
                       battlefield face down. They're 5/5 artifact creatures."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_facedown,
            }),
    )
}

fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}

fn zero_becomes_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dur = Duration::UntilYourNextTurn(ctx.controller);
    vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: dur,
        },
        Effect::SetBasePT {
            target: *id,
            power: 5,
            toughness: 5,
            duration: dur,
        },
    ]
}

fn minus_seven_facedown(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant to put cards from hand onto the battlefield face
    // down as 5/5 artifact creatures.
    Vec::new()
}
