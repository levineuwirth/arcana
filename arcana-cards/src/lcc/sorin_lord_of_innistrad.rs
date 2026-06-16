//! Sorin, Lord of Innistrad — `{2}{W}{B}` Legendary Planeswalker — Sorin,
//! starting loyalty 4.
//!
//! Loyalty abilities:
//! * `+1`: Create a 1/1 black Vampire creature token with lifelink.
//! * `−2`: emblem with "Creatures you control get +1/+0." GAP — anthem emblem
//!   not expressible. Shell declared.
//! * `−6`: Destroy up to three target creatures and/or other planeswalkers;
//!   return each card put into a graveyard this way to the battlefield under
//!   your control. PARTIAL — the destroy of up to three targets is wired; the
//!   "return each to the battlefield under your control" rider (keyed to the
//!   cards destroyed this way) is a GAP (documented).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin, Lord of Innistrad");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 black Vampire creature token with \
                       lifelink.".into(),
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
                effect: plus_one_vampire,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: You get an emblem with \"Creatures you control get \
                       +1/+0.\"".into(),
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
                effect: minus_two_emblem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Destroy up to three target creatures and/or other \
                       planeswalkers. Return each card put into a graveyard \
                       this way to the battlefield under your control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
                    ),
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate_destroy,
            }),
    )
}

/// `+1: Create a 1/1 black Vampire creature token with lifelink.`
fn plus_one_vampire(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").expect("Vampire interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    let token = TokenDefinition {
        name: vampire,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

/// `−2`: anthem emblem.
fn minus_two_emblem(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem with "creatures you control get +1/+0" anthem not expressible.
    Vec::new()
}

/// `−6`: destroy up to three targets (the reanimate-under-your-control rider is
/// GAP'd).
fn ultimate_destroy(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // PARTIAL: "return each card put into a graveyard this way to the
    // battlefield under your control" rider (keyed to cards destroyed this way)
    // is not expressible; only the destruction is wired.
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::DestroyPermanent { target: *id }),
            _ => None,
        })
        .collect()
}
