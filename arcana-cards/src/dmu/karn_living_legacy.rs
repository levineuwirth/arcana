//! Karn, Living Legacy — `{4}` Legendary Planeswalker — Karn, starting
//! loyalty 5.
//!
//! +1: Create a tapped Powerstone token. Modeled as a colorless artifact
//!   token named "Powerstone". The "tapped" entry (no non-attacking
//!   tapped-token primitive) and the token's intrinsic mana ability
//!   ("{T}: Add {C}…") are NOT expressible from `TokenDefinition` —
//!   partially GAP'd; the token is created.
//! −1: Pay any amount of mana. Look at that many cards… — a dynamic-X
//!   variable-mana payment whose effect count depends on the amount paid;
//!   not expressible. GAP (effect returns nothing; correct −1 cost shell).
//! −7: You get an emblem with "Tap an untapped artifact you control: This
//!   emblem deals 1 damage to any target." The emblem's grant is an
//!   ability-granting activated ability the anthem/keyword/filtered
//!   builders can't express. GAP the emblem (correct −7 cost shell).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karn, Living Legacy");
    let karn = reg.interner_mut().intern("Karn");
    let _powerstone = reg.interner_mut().intern("Powerstone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(karn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a tapped Powerstone token.".into(),
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
                effect: plus_one_powerstone,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Pay any amount of mana. Look at that many cards from \
                       the top of your library, then put one of those cards into \
                       your hand and the rest on the bottom of your library in a \
                       random order.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_dig,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"Tap an untapped artifact you \
                       control: This emblem deals 1 damage to any target.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_powerstone(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the token's "enters tapped" state and its intrinsic
    //      "{T}: Add {C}. This mana can't be spent to cast a nonartifact
    //      spell." mana ability are not expressible from TokenDefinition;
    //      the Powerstone artifact token itself is created.
    let powerstone = reg
        .interner()
        .lookup("Powerstone")
        .expect("Powerstone interned at register");
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: powerstone,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn minus_one_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Pay any amount of mana. Look at that many cards…" — a dynamic-X
    //      variable-mana payment whose dug count depends on the amount paid;
    //      not expressible from the demonstrated surface.
    Vec::new()
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the emblem grants an activated ability ("Tap an untapped artifact
    //      you control: deal 1 damage to any target") — an ability-granting
    //      effect the anthem/keyword/filtered emblem builders can't express.
    Vec::new()
}
