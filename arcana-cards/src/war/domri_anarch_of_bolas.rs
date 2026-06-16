//! Domri, Anarch of Bolas — `{1}{R}{G}` Legendary Planeswalker — Domri.
//! Printed starting loyalty 3 (CR 113.3c). Colors R/G.
//!
//! Static: "Creatures you control get +1/+0." — a continuous anthem
//! static, not a loyalty ability. GAP'd here (the loyalty abilities are
//! the focus of this card class; a permanent-sourced anthem would need a
//! separately-registered continuous effect).
//!
//! Keyword: Fight is a templating word (used by the `−2`), not a
//! standalone keyword ability — `keywords: vec![]`.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Add {R} or {G}. Creature spells you cast this turn can't be
//!   countered. — GAP (no "add one mana of a chosen color" choice
//!   primitive, and no this-turn-uncounterable rider in the surface).
//! * `−2`: Target creature you control fights target creature you don't
//!   control. — expressible via `Effect::Fight`.

use arcana_core::effects::Effect;
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
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domri, Anarch of Bolas");
    let domri = reg.interner_mut().intern("Domri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(domri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };
    // GAP: "Creatures you control get +1/+0" static anthem omitted.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R} or {G}. Creature spells you cast this turn can't \
                       be countered.".into(),
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
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Target creature you control fights target creature you \
                       don't control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::You),
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_fight,
            }),
    )
}

/// `+1`: choose-a-color mana + uncounterable rider — GAP.
fn plus_one_mana(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add {R} or {G}" is a mana-color choice with no demonstrated
    // primitive, and "creature spells you cast this turn can't be
    // countered" is a this-turn cast-rider with no surface.
    Vec::new()
}

/// `−2`: your creature fights an opponent's creature.
fn minus_two_fight(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut ids = ctx.targets.targets.iter().filter_map(|c| match c {
        TargetChoice::Object(id) => Some(*id),
        _ => None,
    });
    let (Some(a), Some(b)) = (ids.next(), ids.next()) else {
        return Vec::new();
    };
    vec![Effect::Fight { a, b }]
}
