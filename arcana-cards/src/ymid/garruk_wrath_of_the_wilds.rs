//! Garruk, Wrath of the Wilds — `{2}{G}{G}` Legendary Planeswalker — Garruk,
//! starting loyalty 4.
//!
//! +1: Choose a creature card in your hand. It perpetually gets +1/+1 and
//!     perpetually gains "This spell costs {1} less to cast."
//! −1: Draft a card from Garruk, Wrath of the Wilds's spellbook and put it onto
//!     the battlefield.
//! −6: Until end of turn, creatures you control get +3/+3 and gain trample.
//!
//! GAP: +1 "perpetually gets +1/+1 / costs {1} less" — Alchemy's "perpetually"
//!   modifier on a card in hand is not in the demonstrated Effect surface; the
//!   ability shell is declared with the +1 cost and returns Vec::new().
//! GAP: −1 "draft a card from [this]'s spellbook and put it onto the
//!   battlefield" — Alchemy spellbook drafting is not expressible; the ability
//!   shell is declared with the −1 cost and returns Vec::new().

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Wrath of the Wilds");
    let garruk = reg.interner_mut().intern("Garruk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Choose a creature card in your hand. It perpetually \
                       gets +1/+1 and perpetually gains \"This spell costs {1} \
                       less to cast.\""
                    .into(),
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
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Draft a card from Garruk, Wrath of the Wilds's \
                       spellbook and put it onto the battlefield."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Until end of turn, creatures you control get +3/+3 \
                       and gain trample."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_anthem,
            }),
    )
}

fn plus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Alchemy "perpetually" modifier on a card in hand is not expressible.
    Vec::new()
}

fn minus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Alchemy spellbook drafting is not expressible.
    Vec::new()
}

fn minus_six_anthem(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 3,
            toughness: 3,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample],
        }),
    }]
}
