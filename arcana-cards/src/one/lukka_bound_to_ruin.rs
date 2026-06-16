//! Lukka, Bound to Ruin — `{2}{R}{R/G/P}{G}` Legendary Planeswalker — Lukka,
//! starting loyalty 5. Colors G, R.
//!
//! Compleated ({R/G/P} can be paid with {R}, {G}, or 2 life; if life was paid,
//! this planeswalker enters with two fewer loyalty counters). Compleated is
//! reminder text describing how the hybrid Phyrexian pip is paid — it is NOT in
//! the usable keyword surface and the "enters with two fewer loyalty" rider is
//! not expressible. GAP: Compleated keyword + life-paid loyalty reduction
//! (emit no keyword).
//!
//! +1: Add {R}{G}. Spend this mana only to cast creature spells or activate
//!   abilities of creatures. GAP: restricted mana production from a loyalty
//!   ability is not in the demonstrated Effect surface — ability shell kept
//!   with its +1 cost, empty body.
//! −1: Create a 3/3 green Phyrexian Beast creature token with toxic 1.
//!   Modeled with `Effect::CreateToken` + `KeywordAbility::Toxic(1)`.
//! −4: Lukka deals X damage divided as you choose among any number of target
//!   creatures and/or planeswalkers, where X is the greatest power among
//!   creatures you control as you activate this ability. Modeled with
//!   `Effect::DealDamageDivided` over an any-count creature/planeswalker target
//!   set; X is computed at resolution from `script::power_of` over your
//!   creatures.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lukka, Bound to Ruin");
    let lukka = reg.interner_mut().intern("Lukka");
    // Token strings — interned here so the resolver can `lookup` them.
    let _beast = reg.interner_mut().intern("Beast");
    let _phyrexian = reg.interner_mut().intern("Phyrexian");
    let _token_name = reg.interner_mut().intern("Phyrexian Beast");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lukka);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R/G/P}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R}{G}. Spend this mana only to cast creature \
                       spells or activate abilities of creatures.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_restricted_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Create a 3/3 green Phyrexian Beast creature token \
                       with toxic 1.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: Lukka deals X damage divided as you choose among any \
                       number of target creatures and/or planeswalkers, where X \
                       is the greatest power among creatures you control as you \
                       activate this ability.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(TypeLine(
                            TypeLine::CREATURE | TypeLine::PLANESWALKER,
                        )),
                    ),
                    count: TargetCount::Any,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_divided,
            }),
    )
}

/// `+1: Add {R}{G}. Spend this mana only to cast creature spells or activate
/// abilities of creatures.`
fn plus_one_restricted_mana(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: restricted mana add (mana production from a loyalty ability with a
    // "spend only to cast creature spells / activate creature abilities"
    // spend restriction) is not expressible in the demonstrated Effect surface.
    Vec::new()
}

/// `−1: Create a 3/3 green Phyrexian Beast creature token with toxic 1.`
fn minus_one_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let beast = reg.interner().lookup("Beast").expect("Beast interned");
    let phyrexian = reg.interner().lookup("Phyrexian").expect("Phyrexian interned");
    let name = reg.interner().lookup("Phyrexian Beast").expect("token name interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    subtypes.0.insert(phyrexian);

    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Toxic(1)],
            abilities: vec![],
        },
    }]
}

/// `−4: Lukka deals X damage divided as you choose among any number of target
/// creatures and/or planeswalkers, where X is the greatest power among
/// creatures you control as you activate this ability.`
fn minus_four_divided(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // X = greatest power among creatures you control (0 if you control none).
    let x = state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == ctx.controller && o.is_creature())
        .map(|o| script::power_of(state, o.id))
        .max()
        .unwrap_or(0)
        .max(0) as u32;

    let mut targets = Vec::new();
    for t in &ctx.targets.targets {
        if let TargetChoice::Object(id) = t {
            targets.push(DamageTarget::Object(*id));
        }
    }
    if targets.is_empty() || x == 0 {
        return Vec::new();
    }

    vec![Effect::DealDamageDivided {
        source: ctx.source,
        targets,
        total: x,
    }]
}
