//! Xenagos, the Reveler — `{2}{R}{G}` Legendary Planeswalker — Xenagos,
//! starting loyalty 3.
//!
//! Loyalty abilities:
//! * `+1`: Add X mana in any combination of {R} and/or {G}, where X is the
//!   number of creatures you control. PARTIAL — the amount X is computed at
//!   resolution via `script::count_matching`; the R/G colour split is a player
//!   choice not wired, so X red mana is produced (documented approximation,
//!   same posture as Treasure's colour-choice gap).
//! * `0`: Create a 2/2 red and green Satyr creature token with haste.
//! * `−6`: Exile the top seven cards of your library. You may put any number of
//!   creature and/or land cards from among them onto the battlefield. GAP — a
//!   multi-select exile-then-put-onto-battlefield is not expressible. Shell
//!   declared.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet,
    TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xenagos, the Reveler");
    let xenagos = reg.interner_mut().intern("Xenagos");
    let _satyr = reg.interner_mut().intern("Satyr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(xenagos);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add X mana in any combination of {R} and/or {G}, \
                       where X is the number of creatures you control.".into(),
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
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create a 2/2 red and green Satyr creature token with \
                       haste.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_satyr,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Exile the top seven cards of your library. You may \
                       put any number of creature and/or land cards from among \
                       them onto the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ultimate,
            }),
    )
}

/// `+1: Add X mana, X = creatures you control.` (Colour split approximated as
/// all red.)
fn plus_one_mana(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let x = script::count_matching(state, &ObjectFilter::creature(), ctx.controller);
    if x == 0 {
        return Vec::new();
    }
    let mana = (0..x).map(|_| ManaUnit::plain(ManaColor::Red, ctx.source)).collect();
    vec![Effect::AddMana { player: ctx.controller, mana }]
}

/// `0: Create a 2/2 red and green Satyr creature token with haste.`
fn zero_satyr(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let satyr = reg.interner().lookup("Satyr").expect("Satyr interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(satyr);
    let token = TokenDefinition {
        name: satyr,
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

/// `−6`: exile top seven, may put creature/land cards onto the battlefield.
fn ultimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: exile top-7-then-multi-select-put-onto-battlefield is not
    // expressible from the demonstrated surface.
    Vec::new()
}
