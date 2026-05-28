//! S.N.E.A.K. Dispatcher — `{1}{U}` 2/1 blue Human Spy.
//! "{2}{U}, {T}: Look at the top card of target player's library. If it
//! has an Agents of S.N.E.A.K. watermark, you may reveal it and put it
//! into your hand. Otherwise, put it on your choice of the top or bottom."
//!
//! GAP: "look at top card; if S.N.E.A.K. watermark, put in hand; otherwise
//! top or bottom" — no Effect for watermark check or look-one-card partial.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("S.N.E.A.K. Dispatcher");
    let human = reg.interner_mut().intern("Human");
    let spy = reg.interner_mut().intern("Spy");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(spy);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, {T}: Look at the top card of target player's library. If it has an Agents of S.N.E.A.K. watermark, you may reveal it and put it into your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_card,
            }),
    )
}

fn look_card(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top card; S.N.E.A.K. watermark check; to hand or top/bottom" —
    // watermark mechanic and partial look not modeled.
    Vec::new()
}
