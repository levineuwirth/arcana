//! Grist, the Hunger Tide — `{1}{B}{G}` legendary planeswalker, starting loyalty 3.
//!
//! Static: as long as Grist isn't on the battlefield, it's a 1/1 Insect
//!   (characteristic-defining static, not a loyalty ability — GAP).
//! +1: Create a 1/1 black-and-green Insect token, then mill a card; if an
//!     Insect was milled, add loyalty and repeat (loop GAP).
//! −2: You may sacrifice a creature. When you do, destroy target
//!     creature or planeswalker (optional-sac trigger GAP).
//! −5: Each opponent loses life equal to the number of creature cards in
//!     your graveyard.
//!
//! Scope: +1 expresses the Insect token + a single mill (the
//! Insect-milled loyalty/repeat loop is GAP'd). −5 is fully expressed via
//! a per-opponent LoseLife scaled by creature cards in your graveyard.
//! The −2 optional-sacrifice "when you do" reflexive trigger is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grist, the Hunger Tide");
    let grist = reg.interner_mut().intern("Grist");
    let _insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(grist);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 black and green Insect creature token, \
                       then mill a card. If an Insect card was milled this way, \
                       put a loyalty counter on Grist and repeat this process.".into(),
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
                effect: plus_one_insect,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: You may sacrifice a creature. When you do, destroy \
                       target creature or planeswalker.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_sac,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: Each opponent loses life equal to the number of \
                       creature cards in your graveyard.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_drain,
            }),
    )
}

/// `+1: Create a 1/1 black-and-green Insect token, then mill a card.`
fn plus_one_insect(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let insect = reg.interner().lookup("Insect").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    // GAP: "if an Insect was milled, add loyalty and repeat" loop not
    // expressible — a single token + single mill is emitted.
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: insect,
                colors: ColorSet::black() | ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::Mill { player: ctx.controller, count: 1 },
    ]
}

/// `−2` — optional sacrifice with a reflexive "when you do" trigger.
fn minus_two_sac(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may sacrifice ... when you do, destroy target ..." reflexive
    // trigger not expressible from the demonstrated surface.
    Vec::new()
}

/// `−5: Each opponent loses life equal to creature cards in your graveyard.`
fn minus_five_drain(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creature_filter = ObjectFilter::creature();
    let amount = script::graveyard_matching(state, &creature_filter, ctx.controller, ctx.controller);
    let effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount })
        .collect();
    vec![Effect::Sequence(effects)]
}
