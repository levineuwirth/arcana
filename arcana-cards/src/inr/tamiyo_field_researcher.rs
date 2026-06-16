//! Tamiyo, Field Researcher — `{1}{G}{W}{U}` Legendary Planeswalker — Tamiyo, starting loyalty 4.
//!
//! +1: Choose up to two target creatures. Until your next turn, whenever either of
//!     those creatures deals combat damage, you draw a card. The combat-damage
//!     delayed draw rider is a granted delayed triggered ability not expressible
//!     from the demonstrated activated-ability surface — GAP (effect returns empty;
//!     the up-to-two-creature target shell is preserved).
//! −2: Tap up to two target nonland permanents. They don't untap during their
//!     controller's next untap step. The taps are modeled; the "don't untap next
//!     untap step" rider has no expressible primitive — GAP (taps only).
//! −7: Draw three cards. You get an emblem with "You may cast spells from your hand
//!     without paying their mana costs." Draw three is modeled; the free-cast
//!     emblem static is a rule-altering effect the builders can't express — GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tamiyo, Field Researcher");
    let tamiyo = reg.interner_mut().intern("Tamiyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tamiyo);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let nonland_permanent = ObjectFilter::new().without_types(TypeLine::LAND.into());

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Choose up to two target creatures. Until your next turn, \
                       whenever either of those creatures deals combat damage, you \
                       draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_watch,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Tap up to two target nonland permanents. They don't \
                       untap during their controller's next untap step.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(nonland_permanent),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_tap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Draw three cards. You get an emblem with \"You may cast \
                       spells from your hand without paying their mana costs.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_draw,
            }),
    )
}

fn plus_one_watch(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Until your next turn, whenever either of those creatures deals combat
    //      damage, you draw a card." — granting a duration-bounded delayed combat-
    //      damage triggered ability to the chosen creatures is not expressible
    //      from the demonstrated surface. Targets are still chosen.
    Vec::new()
}

fn minus_two_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "They don't untap during their controller's next untap step." — no
    //      untap-restriction primitive in the demonstrated surface. Taps modeled.
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Tap { target: *id }),
            _ => None,
        })
        .collect()
}

fn minus_seven_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem "You may cast spells from your hand without paying their mana
    //      costs." — a rule-altering cost-replacement static the builders can't
    //      express; omitted. Draw three is modeled.
    vec![Effect::DrawCards { player: ctx.controller, count: 3 }]
}
