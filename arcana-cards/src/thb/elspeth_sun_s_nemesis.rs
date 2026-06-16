//! Elspeth, Sun's Nemesis — `{2}{W}{W}` legendary planeswalker,
//! starting loyalty 5.
//!
//! −1: Up to two target creatures you control each get +2/+1 until end of
//!     turn.
//! −2: Create two 1/1 white Human Soldier creature tokens.
//! −3: You gain 5 life.
//! Escape—{4}{W}{W}, Exile four other cards from your graveyard.
//!
//! # Rules references
//!
//! * CR 113.3c — entering with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (cost = adding/removing loyalty counters).
//! * CR 606.3 — sorcery speed, stack empty, controller-only, once/turn.
//! * CR 704.5i — a planeswalker with 0 loyalty is sacrificed (SBA).
//!
//! # Scope
//!
//! All three loyalty abilities are fully modeled: the `−1` pumps each of
//! up to two chosen creatures you control, the `−2` makes two Human
//! Soldier tokens, the `−3` gains 5 life. The Escape keyword (a graveyard
//! recast mechanic) is NOT in the usable keyword surface for this class —
//! `keywords: vec![]`, noted as a gap.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth, Sun's Nemesis");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let _human = reg.interner_mut().intern("Human");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // Escape ({4}{W}{W}, exile four other cards from graveyard) is not
        // an expressible keyword for this card class — omitted.
        keywords: vec![],
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Up to two target creatures you control each get \
                       +2/+1 until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: Some(ControllerConstraint::You),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create two 1/1 white Human Soldier creature \
                       tokens.".into(),
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
                effect: minus_two_tokens,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: You gain 5 life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_gain_life,
            }),
    )
}

/// `−1: Up to two target creatures you control each get +2/+1 until end of
/// turn.`
fn minus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Pump {
                target: *id,
                power: 2,
                toughness: 1,
                duration: Duration::EndOfTurn,
                keywords: vec![],
            }),
            _ => None,
        })
        .collect()
}

/// `−2: Create two 1/1 white Human Soldier creature tokens.`
fn minus_two_tokens(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").expect("Human interned during register()");
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}

/// `−3: You gain 5 life.`
fn minus_three_gain_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 5 }]
}
