//! Koth of the Hammer — `{2}{R}{R}` Legendary Planeswalker — Koth,
//! starting loyalty 3 — colors R.
//!
//! Oracle text:
//! * `+1`: Untap target Mountain. It becomes a 4/4 red Elemental creature
//!   until end of turn. It's still a land. — `Effect::Untap` of the targeted
//!   Mountain is expressible; the land-animation rider (set 4/4, add Elemental
//!   creature) is not a single demonstrated effect. Best-effort: untap only.
//!   GAP: land→4/4 Elemental creature animation.
//! * `−2`: Add {R} for each Mountain you control. — `Effect::AddMana` with one
//!   red `ManaUnit` per Mountain (`script::count_matching`).
//! * `−5`: You get an emblem with "Mountains you control have '{T}: This land
//!   deals 1 damage to any target.'" — emblem granting a static activated
//!   ability is not expressible as an `EmblemDefinition` payload. GAP.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koth of the Hammer");
    let koth = reg.interner_mut().intern("Koth");
    let _mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(koth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let mountain_target = match reg.interner().lookup("Mountain") {
        Some(s) => ObjectFilter::permanent().with_subtype_sym(s),
        None => ObjectFilter::permanent(),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap target Mountain. It becomes a 4/4 red \
                       Elemental creature until end of turn. It's still a \
                       land.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(mountain_target),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap_mountain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Add {R} for each Mountain you control.".into(),
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
                effect: minus_two_add_red,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: You get an emblem with \"Mountains you control have \
                       '{T}: This land deals 1 damage to any target.'\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_emblem,
            }),
    )
}

/// `+1`: untap target Mountain (animation rider GAP'd).
fn plus_one_untap_mountain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 4/4 red Elemental creature until end of turn" land
    // animation (set base P/T + add type/subtype + set color) is not a single
    // demonstrated effect; untap is wired faithfully.
    match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => vec![Effect::Untap { target: *id }],
        _ => Vec::new(),
    }
}

/// `−2`: add {R} for each Mountain you control.
fn minus_two_add_red(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = match reg.interner().lookup("Mountain") {
        Some(s) => ObjectFilter::permanent()
            .with_subtype_sym(s)
            .controlled_by(ControllerConstraint::You),
        None => return Vec::new(),
    };
    let n = script::count_matching(state, &filter, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    let mana: Vec<ManaUnit> = (0..n)
        .map(|_| ManaUnit::plain(ManaColor::Red, ctx.source))
        .collect();
    vec![Effect::AddMana { player: ctx.controller, mana }]
}

/// `−5`: emblem granting Mountains a tap-for-damage ability.
fn minus_five_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem whose payload is a static grant of an activated ability to
    // Mountains is not expressible via EmblemDefinition.
    Vec::new()
}
