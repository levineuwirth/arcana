//! Ajani, Mentor of Heroes — `{3}{G}{W}` Legendary Planeswalker —
//! Ajani, printed starting loyalty 4 (G/W).
//!
//! Oracle text:
//!   +1: Distribute three +1/+1 counters among one, two, or three
//!       target creatures you control.
//!   +1: Look at the top four cards of your library. You may reveal an
//!       Aura, creature, or planeswalker card from among them and put
//!       it into your hand. Put the rest on the bottom of your library
//!       in any order.
//!   −8: You gain 100 life.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (engine-enforced timing).
//! * CR 704.5i — 0-loyalty state-based sacrifice.
//!
//! # Scope
//!
//! * First `+1` (distribute three +1/+1 counters among up to three
//!   target creatures) — "distribute N among targets" assigns a chosen
//!   per-target split that isn't expressible from the demonstrated
//!   `Effect` surface; GAP'd to an empty effect, `+1` cost preserved.
//! * Second `+1` — `Effect::DigTopN` (look at top 4, optionally take one
//!   matching card to hand, rest to the bottom). PARTIAL: the type-OR
//!   filter covers creature / planeswalker; "Aura" (an enchantment
//!   subtype) can't be OR'd alongside type bits in one `ObjectFilter`,
//!   so Aura is omitted from the reveal filter, and the rest go to the
//!   bottom in random order rather than a chosen order.
//! * `−8` — `Effect::GainLife { amount: 100 }`.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, Mentor of Heroes");
    let ajani = reg.interner_mut().intern("Ajani");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    // "one, two, or three target creatures you control" — up to three.
    let distribute_target = TargetRequirement {
        filter: TargetFilter::Creature,
        count: TargetCount::UpTo(3),
        controller: Some(ControllerConstraint::You),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Distribute three +1/+1 counters among one, \
                       two, or three target creatures you control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![distribute_target],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_distribute,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Look at the top four cards of your library. \
                       You may reveal an Aura, creature, or planeswalker \
                       card from among them and put it into your hand. \
                       Put the rest on the bottom of your library in any \
                       order.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You gain 100 life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_life,
            }),
    )
}

/// `+1: Distribute three +1/+1 counters among one, two, or three target
/// creatures you control.`
fn plus_one_distribute(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "distribute three +1/+1 counters among" the chosen targets
    // requires a per-target count split that isn't expressible from the
    // demonstrated Effect surface.
    Vec::new()
}

/// `+1: Look at the top four cards of your library. You may reveal an
/// Aura, creature, or planeswalker card and put it into your hand. Put
/// the rest on the bottom.`
fn plus_one_dig(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // PARTIAL: creature-or-planeswalker filter; Aura can't be OR'd in.
    let filter = ObjectFilter::permanent()
        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER));
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 4,
        filter: Some(filter),
        rest: DigRest::BottomRandom,
    }]
}

/// `−8: You gain 100 life.`
fn minus_eight_life(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 100 }]
}
