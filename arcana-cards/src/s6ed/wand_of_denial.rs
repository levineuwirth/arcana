//! Wand of Denial — `{2}` artifact.
//! "{T}: Look at the top card of target player's library. If it's a
//! nonland card, you may pay 2 life. If you do, put it into that
//! player's graveyard."
//! Best-effort: an `OptionalPayment` (2 life) gating a 1-card mill of
//! the target player. GAP: the look-at-top peek and the nonland gate
//! are not expressible — the pay/mill choice is offered regardless of
//! whether the top card is a land.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wand of Denial");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Look at the top card of target player's library. If it's a nonland card, you may pay 2 life. If you do, put it into that player's graveyard.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: vec![TargetRequirement::target_player()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: peek_and_mill,
        }),
    )
}

fn peek_and_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "Look at the top card ... If it's a nonland card" — there is
    // no peek-at-top primitive or top-card type gate; the optional
    // 2-life payment milling one card is offered unconditionally.
    vec![Effect::OptionalPayment {
        chooser: ctx.controller,
        cost: OptionalPaymentKind::Life(2),
        then: Box::new(Effect::Mill { player: *p, count: 1 }),
        else_effect: None,
    }]
}
