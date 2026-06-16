//! Ajani Goldmane — `{2}{W}{W}` Legendary Planeswalker — Ajani,
//! starting loyalty 4.
//!
//! +1: You gain 2 life.
//! −1: Put a +1/+1 counter on each creature you control. Those creatures gain
//!     vigilance until end of turn.
//! −6: Create a white Avatar creature token with "this token's power and
//!     toughness are each equal to your life total."
//!     GAP: the token's characteristic-defining P/T (= your life total) is not
//!          expressible; we mint a 0/0 white Avatar token as a best effort.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani Goldmane");
    let ajani = reg.interner_mut().intern("Ajani");
    // Intern the Avatar token subtype up front so the -6 resolver can look it up.
    let _avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You gain 2 life.".into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Put a +1/+1 counter on each creature you control. \
                       Those creatures gain vigilance until end of turn.".into(),
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
                effect: minus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Create a white Avatar creature token. It has \"This \
                       token's power and toughness are each equal to your life \
                       total.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 2 }]
}

fn minus_one(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = arcana_core::targets::ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    ids.into_iter()
        .flat_map(|id| {
            vec![
                Effect::AddCounters {
                    target: id,
                    kind: CounterKind::PlusOnePlusOne,
                    count: 1,
                },
                Effect::GrantKeyword {
                    target: id,
                    keyword: KeywordAbility::Vigilance,
                    duration: Duration::EndOfTurn,
                },
            ]
        })
        .collect()
}

fn minus_six(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's P/T = your life total (a characteristic-defining
    // ability) is not expressible. Mint a 0/0 white Avatar token as a best
    // effort so the loyalty path is exercised.
    let avatar = reg.interner().lookup("Avatar")
        .expect("Avatar interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(avatar);
    let token = TokenDefinition {
        name: avatar,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}
