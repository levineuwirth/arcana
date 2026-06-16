//! Huatli, Warrior Poet — `{3}{R}{W}` legendary planeswalker, starting
//! loyalty 3. Subtype Huatli.
//!
//! # Loyalty abilities
//!
//! * `+2`: You gain life equal to the greatest power among creatures
//!   you control. GAP — `Effect::GainLife` takes a fixed `u32`; a
//!   board-derived dynamic life amount is not expressible from the
//!   demonstrated surface. Ability shell declared at the correct cost.
//! * `0`: Create a 3/3 green Dinosaur creature token with trample.
//!   (Effect::CreateToken.)
//! * `−X`: Huatli deals X damage divided among any number of target
//!   creatures. OMITTED — `−X` dynamic loyalty cost is not expressible
//!   (`remove_self_counter` is a fixed `u32`).
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 / 606.3 — loyalty abilities.
//! * CR 704.5i — 0-loyalty state-based sacrifice.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huatli, Warrior Poet");
    let huatli = reg.interner_mut().intern("Huatli");
    let _dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(huatli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You gain life equal to the greatest power among \
                       creatures you control.".into(),
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
                text: "0: Create a 3/3 green Dinosaur creature token with \
                       trample.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_dino,
            }),
        // `−X` ability omitted: dynamic-X loyalty cost is not expressible.
    )
}

/// `+2: You gain life equal to the greatest power among creatures you
/// control.`
fn plus_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic life amount (greatest power among your creatures) is
    // not expressible; `Effect::GainLife` takes a fixed `u32`.
    Vec::new()
}

/// `0: Create a 3/3 green Dinosaur creature token with trample.`
fn make_dino(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dinosaur = reg.interner().lookup("Dinosaur").expect("Dinosaur interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    let token = TokenDefinition {
        name: dinosaur,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}
