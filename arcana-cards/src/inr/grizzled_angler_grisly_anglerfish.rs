//! Grizzled Angler // Grisly Anglerfish — `{2}{U}` transform creature.
//! Front: Human 2/3.
//!   {T}: Mill two cards. Then if there is a colorless creature card in your
//!   graveyard, transform this creature.
//! Back: Eldrazi Fish (no mana cost), colorless.
//!   {6}: Creatures your opponents control attack this turn if able.
//!
//! GAP: "if there is a colorless creature card in your graveyard" — the front {T}
//!      ability transforms unconditionally. The condition would have to be checked
//!      AFTER the Mill executes, but `Effect::Conditional`'s `Condition` enum has
//!      no filtered-graveyard variant and `Condition::Custom(fn(&GameState)->bool)`
//!      receives no controller, so "a colorless creature card in *your* graveyard"
//!      cannot be expressed post-mill.
//! Back face "{6}: Creatures your opponents control attack this turn if able"
//! installs a board-wide filtered must-attack (CR 508.1a) on opponents'
//! creatures until end of turn (face-gated to the back).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grizzled Angler");
    let human_sub = reg.interner_mut().intern("Human");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(human_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Grisly Anglerfish");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let fish_sub = reg.interner_mut().intern("Fish");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(eldrazi_sub);
    back_subs.0.insert(fish_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // {T}: Mill two, then transform (front face only).
            // GAP: "if there is a colorless creature card in your graveyard"
            //      condition not checked; transforms unconditionally.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Mill two cards. Then if there is a colorless creature card \
                       in your graveyard, transform this creature.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: tap_mill_transform,
            })
            // Back (face 1): {6}: Creatures your opponents control attack
            // this turn if able.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}: Creatures your opponents control attack this turn \
                       if able.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: opponents_creatures_must_attack,
            }),
    )
}

fn opponents_creatures_must_attack(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_must_attack(
            ctx.source,
            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
            Duration::EndOfTurn,
        ),
    }]
}

fn tap_mill_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Mill { player: ctx.controller, count: 2 },
        Effect::Transform { target: ctx.source },
    ]
}
