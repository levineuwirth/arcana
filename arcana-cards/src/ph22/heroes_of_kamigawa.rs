//! Heroes of Kamigawa — `{1}{W}{U}{B}` Legendary Planeswalker — Kaito
//! Wanderer, starting loyalty 5. W/U/B.
//!
//! Oracle text:
//! * Each spell you cast with a name printed in a Kamigawa expansion
//!   costs {1} less to cast.
//! * `+2`: Open up to one sealed Kamigawa booster pack and shuffle those
//!   cards into your booster pile. Look at the top four cards of your
//!   booster pile. Put two of those cards into your hand and the rest
//!   into your graveyard.
//! * `−3`: Return a creature card with a name printed in a Kamigawa
//!   expansion from your graveyard to the battlefield.
//! * Heroes of Kamigawa can be your commander.
//!
//! # Scope
//!
//! This is a "booster pile" mechanic card (Unfinity-adjacent): the
//! cost-reduction static, the "open a sealed booster pack / booster
//! pile" `+2`, and the set-name-restricted graveyard reanimation `−3`
//! are all bespoke or rely on data (printed-set membership, booster
//! piles) the demonstrated surface doesn't carry. Both loyalty
//! abilities are declared with their correct costs and GAP'd effects.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heroes of Kamigawa");
    // Type line subtype is the two-word "Kaito Wanderer".
    let subtype = reg.interner_mut().intern("Kaito Wanderer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Open up to one sealed Kamigawa booster pack and \
                       shuffle those cards into your booster pile. Look at \
                       the top four cards of your booster pile. Put two of \
                       those cards into your hand and the rest into your \
                       graveyard.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Return a creature card with a name printed in a \
                       Kamigawa expansion from your graveyard to the \
                       battlefield.".into(),
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
                effect: minus_three,
            }),
    )
}

/// `+2`: booster-pile mechanic.
fn plus_two(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "open a sealed booster pack / booster pile" is a bespoke
    // mechanic with no engine surface.
    Vec::new()
}

/// `−3`: set-name-restricted graveyard reanimation.
fn minus_three(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "a creature card with a name printed in a Kamigawa expansion"
    // is a printed-set-membership filter the surface doesn't carry.
    Vec::new()
}
