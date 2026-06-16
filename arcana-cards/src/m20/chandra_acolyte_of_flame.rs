//! Chandra, Acolyte of Flame — `{1}{R}{R}` Legendary Planeswalker —
//! Chandra, starting loyalty 4.
//!
//! * `0`: Put a loyalty counter on each red planeswalker you control.
//!   Resolution-time `script::ids_matching` (red planeswalkers you
//!   control) + per-id `AddCounters(Loyalty)`.
//! * `0`: Create two 1/1 red Elemental creature tokens with haste;
//!   sacrifice them at the beginning of the next end step. Two
//!   `Effect::CreateTokenSacEot`.
//! * `−2`: Cast target instant or sorcery (mana value ≤ 3) from your
//!   graveyard, exiling it instead of it dying. GAP'd (graveyard-
//!   targeting / cast-from-graveyard rider not expressible here).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Acolyte of Flame");
    let chandra = reg.interner_mut().intern("Chandra");
    let _elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Put a loyalty counter on each red planeswalker you \
                       control.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_pump_walkers,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create two 1/1 red Elemental creature tokens. They \
                       gain haste. Sacrifice them at the beginning of the next \
                       end step.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_elementals,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: You may cast target instant or sorcery card with mana \
                       value 3 or less from your graveyard. If that spell would \
                       be put into your graveyard, exile it instead.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_flashback,
            }),
    )
}

fn zero_pump_walkers(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let walkers = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::PLANESWALKER.into())
            .with_colors(ColorSet::red())
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    walkers
        .into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::Loyalty,
            count: 1,
        })
        .collect()
}

fn zero_elementals(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental")
        .expect("Elemental interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: elemental,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };
    vec![
        Effect::CreateTokenSacEot { controller: ctx.controller, token: token.clone() },
        Effect::CreateTokenSacEot { controller: ctx.controller, token },
    ]
}

fn minus_two_flashback(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cast a targeted instant/sorcery (mv ≤ 3) from your graveyard —
    // graveyard-targeting / cast-from-graveyard rider not expressible.
    Vec::new()
}
