//! Nissa, Worldwaker — `{3}{G}{G}` Legendary Planeswalker — Nissa, starting loyalty 3.
//!
//! +1: Target land you control becomes a 4/4 Elemental creature with
//!   trample. It's still a land. IMPLEMENTED via AddType(CREATURE) +
//!   SetBasePT(4/4) + GrantKeyword(Trample), all EndOfTurn (land type
//!   kept since AddType is additive). NOTE: the printed animation is
//!   permanent, not until-end-of-turn; the demonstrated durations only
//!   offer EndOfTurn/WhileSourceOnBattlefield, so EndOfTurn is the
//!   closest expressible. (Elemental subtype grant — GAP.)
//! +1: Untap up to four target Forests. IMPLEMENTED via up-to-4 targets +
//!   per-target Untap.
//! −7: Search your library for any number of basic land cards, put them
//!   onto the battlefield, then shuffle. Those lands become 4/4 Elemental
//!   creatures with trample. They're still lands. GAP: no library-search /
//!   put-onto-battlefield primitive in the demonstrated Effect surface.
//!   Ability shell keeps the correct −7 cost, GAP'd body.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Worldwaker");
    let nissa = reg.interner_mut().intern("Nissa");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let land_target = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent()
                .with_types(TypeLine::LAND.into())
                .controlled_by(ControllerConstraint::You),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    let forest_target = TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter::permanent().with_subtype_sym(forest)),
        count: TargetCount::UpTo(4),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target land you control becomes a 4/4 Elemental \
                       creature with trample. It's still a land.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![land_target],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_animate_land,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap up to four target Forests.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![forest_target],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap_forests,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Search your library for any number of basic land \
                       cards, put them onto the battlefield, then shuffle. Those \
                       lands become 4/4 Elemental creatures with trample. They're \
                       still lands.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_search,
            }),
    )
}

/// `+1` — target land becomes a 4/4 Elemental with trample (still a land).
fn plus_one_animate_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    // GAP: "Elemental" creature-type grant has no demonstrated Effect.
    vec![
        Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: id,
            power: 4,
            toughness: 4,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        },
    ]
}

/// `+1` — untap up to four target Forests.
fn plus_one_untap_forests(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Untap { target: *id }),
            _ => None,
        })
        .collect()
}

/// `−7` — search for basic lands, put onto battlefield, animate them.
fn minus_seven_search(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no library-search / put-onto-battlefield primitive in the
    // demonstrated Effect surface; the subsequent animation is therefore
    // unreachable too. Ability shell retains the correct −7 cost.
    Vec::new()
}
