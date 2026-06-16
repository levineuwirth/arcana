//! Nissa Revane — `{2}{G}{G}` Legendary Planeswalker — Nissa, starting
//! loyalty 3.
//!
//! +1: Search your library for a card named Nissa's Chosen, put it onto
//!     the battlefield, then shuffle.
//! +1: You gain 2 life for each Elf you control.
//! −7: Search your library for any number of Elf creature cards, put them
//!     onto the battlefield, then shuffle. (Partial — see resolver: only a
//!     single Elf creature is tutored; "any number" isn't expressible.)

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa Revane");
    let nissa = reg.interner_mut().intern("Nissa");
    let _chosen = reg.interner_mut().intern("Nissa's Chosen");
    let _elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Search your library for a card named Nissa's Chosen, put it onto the battlefield, then shuffle.".into(),
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
                effect: plus_one_tutor_chosen,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain 2 life for each Elf you control.".into(),
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
                effect: plus_one_gain_life,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Search your library for any number of Elf creature cards, put them onto the battlefield, then shuffle.".into(),
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
                effect: minus_seven_tutor_elves,
            }),
    )
}

fn plus_one_tutor_chosen(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let chosen = reg.interner().lookup("Nissa's Chosen").expect("name interned");
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter {
            name: Some(chosen),
            ..Default::default()
        },
        tapped: false,
    }]
}

fn plus_one_gain_life(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elf_filter =
        script::subtype_filter(reg, "Elf").controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &elf_filter, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::GainLife { player: ctx.controller, amount: 2 * n }]
}

fn minus_seven_tutor_elves(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: "any number of Elf creature cards" — TutorToBattlefield fetches
    //          a SINGLE matching card, so only one Elf creature is put onto
    //          the battlefield. The multi-card "any number" search isn't
    //          expressible from the demonstrated surface.
    let elf = reg.interner().lookup("Elf").expect("Elf interned");
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter {
            types: Some(TypeLine::CREATURE.into()),
            subtypes: Some(vec![elf]),
            ..Default::default()
        },
        tapped: false,
    }]
}
