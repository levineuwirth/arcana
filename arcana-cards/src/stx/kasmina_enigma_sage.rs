//! Kasmina, Enigma Sage — `{1}{G}{U}` Legendary Planeswalker — Kasmina, starting loyalty 5.
//!
//! Static: "Each other planeswalker you control has the loyalty abilities of
//!   Kasmina." — an ability-granting continuous effect; not a loyalty ability and
//!   not expressible from the demonstrated surface. GAP (static, not modeled).
//!
//! +2: Scry 1.
//! −X: Create a 0/0 green and blue Fractal creature token; put X +1/+1 counters
//!   on it. GAP: dynamic-X loyalty cost is not expressible (`remove_self_counter`
//!   is a fixed u32). Ability omitted.
//! −8: Search your library for an instant or sorcery card that shares a color
//!   with this planeswalker, exile that card, then shuffle. You may cast it
//!   without paying its mana cost. GAP: "search-exile-then-cast-from-exile-free"
//!   chain with a shares-a-color predicate is not expressible; effect GAP'd, but
//!   the ability shell is declared at the correct −8 cost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kasmina, Enigma Sage");
    let kasmina = reg.interner_mut().intern("Kasmina");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kasmina);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Scry 1.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_scry,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: Search your library for an instant or sorcery card \
                       that shares a color with this planeswalker, exile that \
                       card, then shuffle. You may cast that card without paying \
                       its mana cost.".into(),
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
                effect: minus_eight_gap,
            }),
    )
}

/// `+2: Scry 1.`
fn plus_two_scry(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: ctx.controller, count: 1 }]
}

/// `-8` — GAP: search + exile + cast-from-exile-free with a shares-a-color
/// predicate is not expressible from the demonstrated surface.
fn minus_eight_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: tutored-card exile + free cast + shares-a-color filter not expressible.
    Vec::new()
}
