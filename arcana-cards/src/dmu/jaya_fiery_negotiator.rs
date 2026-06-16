//! Jaya, Fiery Negotiator — `{2}{R}{R}` Legendary Planeswalker — Jaya, starting loyalty 4.
//! +1: Create a 1/1 red Monk creature token with prowess. IMPLEMENTED (prowess omitted — not a
//!     KeywordAbility variant; GAP note).
//! −1: Exile top two, choose one, may play this turn. IMPLEMENTED via Effect::ImpulseExile
//!     { count: 2 } (close approximation — may slightly overshoot by allowing both to be played).
//! −2: Choose target creature an opponent controls; delayed "whenever you attack this turn" combat
//!     trigger dealing damage = attackers. GAP — delayed combat trigger not expressible.
//! −8: You get an emblem with "Whenever you cast a red instant or sorcery spell, copy it twice."
//!     Emblem emitted; copy-spell not buildable so the emblem's ability is GAP'd (empty).

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jaya, Fiery Negotiator");
    let sub = reg.interner_mut().intern("Jaya");
    let monk = reg.interner_mut().intern("Monk");
    let _emblem = reg.interner_mut().intern("Jaya, Fiery Negotiator emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let _ = monk;

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
                text: "−1: Exile the top two cards of your library. Choose one of them. You may \
                       play that card this turn."
                    .into(),
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
                text: "−2: Choose target creature an opponent controls. Whenever you attack this \
                       turn, Jaya, Fiery Negotiator deals damage equal to the number of attacking \
                       creatures to that creature."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::CREATURE.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_attack_trigger,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You get an emblem with \"Whenever you cast a red instant or sorcery \
                       spell, copy it twice. You may choose new targets for the copies.\""
                    .into(),
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

/// `+1` — create a 1/1 red Monk token with prowess (prowess omitted — not a KeywordAbility).
fn plus_one_monk(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let monk = reg.interner().lookup("Monk").expect("Monk interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(monk);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: monk,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            // GAP: prowess is not a KeywordAbility variant — omitted.
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−1` — exile top two, play one this turn (ImpulseExile approximation; may slightly overshoot).
fn minus_one_impulse(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // ImpulseExile allows playing exiled cards this turn. Close approximation to "choose one of
    // them, you may play that card" — may overshoot by permitting more than one to be played.
    vec![Effect::ImpulseExile {
        player: ctx.controller,
        count: 2,
    }]
}

/// `−2` — delayed "whenever you attack" combat trigger dealing damage to chosen creature.
fn minus_two_attack_trigger(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: a delayed "whenever you attack this turn, deal damage equal to the number of attacking
    // creatures to that creature" combat trigger is not expressible from this Effect surface.
    Vec::new()
}

/// `−8` — emblem (copy-spell twice not buildable; emit emblem shell with GAP'd ability).
fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Jaya, Fiery Negotiator emblem")
        .expect("emblem name interned");
    // GAP: "copy it twice" (spell-copy) is not expressible; emit the emblem with empty abilities.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![],
        },
    }]
}
