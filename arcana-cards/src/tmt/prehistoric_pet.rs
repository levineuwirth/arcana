//! Prehistoric Pet — `{W}` 1/2 Dinosaur Ninja.
//!
//! * "This creature can't be blocked by creatures with greater power."
//!   — a static blocker restriction comparing the blocker's power to
//!   this creature's; not expressible with the demonstrated primitives.
//!   GAP'd.
//! * `{1}{W}, {T}: Return another target creature you control to its
//!   owner's hand. Activate only during your turn.` (The "activate only
//!   during your turn" timing restriction has no demonstrated
//!   `ActivationCost` field — it is a sorcery-speed-like timing gate,
//!   GAP'd; the bounce itself is emitted.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Prehistoric Pet");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "can't be blocked by creatures with greater power" — a
    // power-comparison blocker restriction with no demonstrated primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{W}, {T}: Return another target creature you control to its owner's hand."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: bounce_own_creature,
        }),
    )
}

fn bounce_own_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}
