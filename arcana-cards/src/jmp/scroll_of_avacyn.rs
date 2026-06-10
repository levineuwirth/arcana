//! Scroll of Avacyn — {1} artifact. "{1}, Sacrifice this artifact:
//! Draw a card. If you control an Angel, you gain 5 life." The Angel
//! check is a resolution-time board count.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scroll of Avacyn");
    // Pre-intern "Angel" so the resolver's subtype_filter lookup resolves.
    let _angel = reg.interner_mut().intern("Angel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, Sacrifice this artifact: Draw a card. If you \
                       control an Angel, you gain 5 life."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_and_maybe_gain,
            },
        ),
    )
}

fn draw_and_maybe_gain(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects =
        vec![Effect::DrawCards { player: ctx.controller, count: 1 }];
    let angels = script::subtype_filter(reg, "Angel")
        .controlled_by(ControllerConstraint::You);
    if script::count_matching(state, &angels, ctx.controller) > 0 {
        effects.push(Effect::GainLife { player: ctx.controller, amount: 5 });
    }
    effects
}
