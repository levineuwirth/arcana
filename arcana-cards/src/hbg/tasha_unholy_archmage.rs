//! Tasha, Unholy Archmage — `{2}{U}{B}` Legendary Planeswalker — Tasha,
//! starting loyalty 3.
//!
//! Oracle text:
//! * `+1`: Until your next turn, whenever a creature attacks you or
//!   Tasha, Unholy Archmage, put a -1/-1 counter on that creature.
//! * `−2`: Target opponent puts a creature card of their choice from
//!   their graveyard onto the battlefield under your control. That
//!   creature gains ward {2}.
//! * `−6`: Target opponent reveals cards from the top of their library
//!   until they reveal three creature cards. Put those cards onto the
//!   battlefield under your control. That player puts the rest into
//!   their graveyard.
//!
//! # Rules references
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities.
//!
//! # Scope
//! All three ability shells are declared with their correct loyalty
//! costs, but none of the effects are expressible from the demonstrated
//! `Effect` surface:
//! * `+1` is a floating "until your next turn" delayed triggered
//!   ability granted by a loyalty effect — not expressible.
//! * `−2` reanimates from an OPPONENT's graveyard under YOUR control
//!   and grants ward — graveyard-reanimate-under-your-control is not
//!   expressible.
//! * `−6` is a reveal-until-three-creatures-then-put-onto-battlefield
//!   bespoke ultimate — not expressible.
//! Planeswalker animation / emblems do not apply to this card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tasha, Unholy Archmage");
    let tasha = reg.interner_mut().intern("Tasha");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tasha);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, whenever a creature \
                       attacks you or Tasha, Unholy Archmage, put a -1/-1 \
                       counter on that creature.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target opponent puts a creature card of their \
                       choice from their graveyard onto the battlefield \
                       under your control. That creature gains ward {2}.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Target opponent reveals cards from the top of \
                       their library until they reveal three creature \
                       cards. Put those cards onto the battlefield under \
                       your control. That player puts the rest into their \
                       graveyard.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six,
            }),
    )
}

fn plus_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: granting a floating "until your next turn" delayed triggered
    // ability from a loyalty effect is not expressible.
    Vec::new()
}

fn minus_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: reanimating a creature from an opponent's graveyard onto the
    // battlefield under your control + granting ward is not expressible.
    Vec::new()
}

fn minus_six(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: reveal-until-three-creatures then put onto battlefield under
    // your control is a bespoke ultimate not expressible.
    Vec::new()
}
