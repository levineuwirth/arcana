//! Jaya, Fiery Negotiator — `{2}{R}{R}` Legendary Planeswalker — Jaya,
//! starting loyalty 5.
//!
//! +1: Create a 1/1 red Monk creature token with prowess. (Prowess isn't an
//!     available keyword — the token is created without it; GAP.)
//! −1: Exile the top two cards of your library. Choose one of them. You may
//!     play that card this turn. (Partial — ImpulseExile lets you play
//!     EITHER exiled card; the "choose one to play" restriction isn't
//!     expressible.)
//! −2: Choose target creature an opponent controls. Whenever you attack this
//!     turn, Jaya deals damage equal to the number of attacking creatures to
//!     that creature. (GAP — floating attack-triggered dynamic damage.)
//! −8: You get an emblem with "Whenever you cast a red instant or sorcery
//!     spell, copy it twice. You may choose new targets for the copies."
//!     (GAP — emblem.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaya, Fiery Negotiator");
    let jaya = reg.interner_mut().intern("Jaya");
    let _monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jaya);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
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
                text: "+1: Create a 1/1 red Monk creature token with prowess.".into(),
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
                effect: plus_one_monk,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Exile the top two cards of your library. Choose one of them. You may play that card this turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_impulse,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Choose target creature an opponent controls. Whenever you attack this turn, Jaya deals damage equal to the number of attacking creatures to that creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_attack_watcher,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever you cast a red instant or sorcery spell, copy it twice. You may choose new targets for the copies.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_one_monk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: prowess isn't an available keyword — the Monk is created without it.
    let monk = reg.interner().lookup("Monk").expect("Monk interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(monk);
    let token = TokenDefinition {
        name: monk,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_one_impulse(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: oracle exiles two and lets you play ONE of them; ImpulseExile
    //          exiles two and lets you play EITHER this turn. The
    //          "choose one to play" restriction isn't expressible.
    vec![Effect::ImpulseExile { player: ctx.controller, count: 2 }]
}

fn minus_two_attack_watcher(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Whenever you attack this turn, Jaya deals damage equal to the
    //      number of attacking creatures to that creature" — a floating
    //      attack-triggered effect with a remembered target + dynamic damage
    //      isn't expressible from the demonstrated surface (the chosen target
    //      can't be threaded into a ScheduleFloatingTrigger effect fn).
    Vec::new()
}

fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a cast-triggered copy-twice rider — not expressible.
    Vec::new()
}
