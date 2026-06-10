//! Crooked Scales — `{4}` artifact.
//! "{4}, {T}: Flip a coin. If you win the flip, destroy target
//! creature an opponent controls. If you lose the flip, destroy
//! target creature you control unless you pay {3} and repeat this
//! process." Modeled with `Effect::FlipCoin`; the lose branch is an
//! `OptionalPayment` whose punishment is the destroy.
//!
//! GAP: "and repeat this process" — re-running the flip after paying
//! {3} is not expressible (no loop primitive); paying simply saves
//! the creature with no repeat.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crooked Scales");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Flip a coin. If you win the flip, destroy \
                       target creature an opponent controls. If you lose \
                       the flip, destroy target creature you control unless \
                       you pay {3} and repeat this process."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: weigh_the_scales,
            },
        ),
    )
}

fn weigh_the_scales(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(theirs)) = ctx.targets.targets.first()
    else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(yours)) = ctx.targets.targets.get(1) else {
        return Vec::new();
    };
    // GAP: "and repeat this process" — no loop primitive; paying {3}
    // simply spares the creature without re-flipping.
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::DestroyPermanent { target: *theirs }),
        lose: Some(Box::new(Effect::OptionalPayment {
            chooser: ctx.controller,
            cost: OptionalPaymentKind::Mana(
                ManaCost::parse("{3}").expect("valid cost"),
            ),
            then: Box::new(Effect::Sequence(vec![])),
            else_effect: Some(Box::new(Effect::DestroyPermanent {
                target: *yours,
            })),
        })),
    }]
}
