//! Sarkhan Unbroken — `{2}{G}{U}{R}` Legendary Planeswalker — Sarkhan,
//! starting loyalty 5.
//!
//! * `+1`: Draw a card, then add one mana of any color. The draw is
//!   expressed; the "add one mana of any color" rider is GAP'd (the
//!   color choice is not expressible — `Effect::AddMana` takes fixed
//!   `ManaUnit`s).
//! * `−2`: Create a 4/4 red Dragon creature token with flying.
//!   `Effect::CreateToken`.
//! * `−8`: Search your library for any number of Dragon creature cards,
//!   put them onto the battlefield, then shuffle. Best-effort via
//!   `Effect::TutorToBattlefield` (Phase 1 fetches one matching card;
//!   the "any number" multiplicity is not faithfully expressible).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan Unbroken");
    let sarkhan = reg.interner_mut().intern("Sarkhan");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sarkhan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw a card, then add one mana of any color.".into(),
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
                effect: plus_one_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a 4/4 red Dragon creature token with flying.".into(),
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
                effect: minus_two_dragon,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Search your library for any number of Dragon creature \
                       cards, put them onto the battlefield, then shuffle.".into(),
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
                effect: minus_eight_tutor,
            }),
    )
}

fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "add one mana of any color" — the color choice is not
    // expressible (Effect::AddMana takes fixed ManaUnits). Draw is emitted.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn minus_two_dragon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon")
        .expect("Dragon interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let token = TokenDefinition {
        name: dragon,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_eight_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon")
        .expect("Dragon interned during register()");
    // Best-effort: tutors one Dragon creature card to the battlefield. The
    // "any number" multiplicity is not faithfully expressible.
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_subtype_sym(dragon),
        tapped: false,
    }]
}
