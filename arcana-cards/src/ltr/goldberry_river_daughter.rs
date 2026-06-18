//! Goldberry, River-Daughter — `{1}{U}` Legendary 1/3 Nymph.
//!
//! {T}: Move a counter of each kind not on Goldberry from another
//! target permanent you control onto Goldberry.
//! {U}, {T}: Move one or more counters from Goldberry onto another
//! target permanent you control. If you do, draw a card.
//!
//! Both abilities move counters between specific permanents. The
//! demonstrated Effect catalog has no counter-MOVE primitive, and
//! "another target permanent you control" (self-excluded targeting)
//! is not expressible with the shown ObjectFilter API — so the
//! effects are GAP'd; the ability shells (cost + target) are emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goldberry, River-Daughter");
    let nymph = reg.interner_mut().intern("Nymph");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nymph);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Move a counter of each kind not on Goldberry from another target permanent you control onto Goldberry.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: move_in,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, {T}: Move one or more counters from Goldberry onto another target permanent you control. If you do, draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: move_out_draw,
            }),
    )
}

fn move_in(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no counter-MOVE Effect primitive in the demonstrated catalog
    // (would need Effect::MoveCounter for "a counter of each kind").
    Vec::new()
}

fn move_out_draw(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no counter-MOVE Effect primitive in the demonstrated catalog
    // ("move one or more counters from Goldberry onto target"); the
    // conditional draw rider depends on that move resolving.
    Vec::new()
}
