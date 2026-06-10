//! Slate of Ancestry — `{4}` artifact.
//! "{4}, {T}, Discard your hand: Draw a card for each creature you
//! control."
//! Cost: {4} mana + tap + discard-your-hand on one `ActivationCost`;
//! the draw amount is computed at resolution from the board.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slate of Ancestry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{4}, {T}, Discard your hand: Draw a card for each creature you control.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                tap: true,
                discard_hand: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: draw_per_creature,
        }),
    )
}

fn draw_per_creature(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let count = script::count_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards { player: ctx.controller, count }]
}
